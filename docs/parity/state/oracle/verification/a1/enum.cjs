const ts=require(process.argv[2]+"/node_modules/typescript");
const idx=process.argv[3];
const prog=ts.createProgram([idx],{target:ts.ScriptTarget.ES2020,strict:false});
const ch=prog.getTypeChecker();
const mod=ch.getSymbolAtLocation(prog.getSourceFile(idx));
const out=[];
const isInternal=d=>{const sf=d.getSourceFile();const r=ts.getLeadingCommentRanges(sf.text,d.getFullStart())||[];return r.some(c=>/@internal/.test(sf.text.slice(c.pos,c.end)))};
const isPriv=d=>{const f=ts.getCombinedModifierFlags(d);return (f&(ts.ModifierFlags.Private|ts.ModifierFlags.Protected))||(d.name&&ts.isPrivateIdentifier(d.name))};
for(let s of ch.getExportsOfModule(mod)){
  const name=s.name; let t=s; if(t.flags&ts.SymbolFlags.Alias) t=ch.getAliasedSymbol(t);
  out.push(name);
  const decl=t.declarations[0];
  const own=(p,owner)=>p.declarations&&p.declarations.some(d=>owner.includes(d.parent)&&!isInternal(d)&&!isPriv(d));
  const owners=t.declarations;
  if(t.flags&(ts.SymbolFlags.Class|ts.SymbolFlags.Interface)){
    const it=ch.getDeclaredTypeOfSymbol(t);
    for(const p of ch.getPropertiesOfType(it)) if(own(p,owners)) out.push(name+"."+p.name);
    // ctor param properties
    for(const d of owners) if(d.members) for(const m of d.members) if(ts.isConstructorDeclaration(m)) for(const prm of m.parameters) if(prm.modifiers&&!isPriv(prm)&&!isInternal(prm)&&!out.includes(name+"."+prm.name.text)) out.push(name+"."+prm.name.text);
  }
  if(t.flags&ts.SymbolFlags.Class){
    const st=ch.getTypeOfSymbolAtLocation(t,decl);
    for(const p of ch.getPropertiesOfType(st)) if(p.name!=="prototype"&&own(p,owners)) out.push(name+"^"+p.name);
  }
  if(t.flags&ts.SymbolFlags.Enum){ for(const m of decl.members) out.push(name+"."+m.name.getText()); }
  if(t.flags&ts.SymbolFlags.TypeAlias && ts.isTypeLiteralNode(decl.type)) for(const m of decl.type.members) out.push(name+"."+m.name.getText());
}
console.log(out.join("\n"));
