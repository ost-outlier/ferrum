# A2 raw inventory: `ferrum-state` public items

> What: A2 raw inventory of Ferrum public items. Versions: cm6 `@codemirror/state` 6.7.6 (`CodeMirror6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`ferrum/crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `ferrum/docs/parity/README.md`.

Generated from rustdoc JSON (`RUSTC_BOOTSTRAP=1 cargo rustdoc -p ferrum-state -- -Z unstable-options --output-format json`, format 61; script `../oracle/tools/ferrum-pub.py`), plus hand-listed non-derived std trait impls (rustdoc JSON lists them, the script records them per type) and the items the `ferrum` facade exposes.

Row kinds: modules, structs, enums, variants, `pub` fields, traits, trait methods, inherent `pub` methods, impls of crate traits, consts, statics, type aliases, crate-root re-exports (`lib.rs`), non-derived std trait impls with behavior (`Display`, `Debug`, `Default`, `From`), facade re-exports. Derived impls (`Clone`, `PartialEq`, `Eq`, `Hash`, `Ord`, `Debug`, `Default`) are recorded in the type's row in `ferrum/docs/parity/state/api-matrix.md`, not as rows.

**Total: 391 rows** = 352 rustdoc items (48 of them crate-root re-exports, 10 modules) + 19 std trait impls + 20 facade re-exports.

| id | Ferrum item | kind | file:line | chunk |
|---|---|---|---|---|
| state.fe.001 | `ferrum_state::change` | module | `ferrum/crates/ferrum-state/src/change.rs:1` | main |
| state.fe.002 | `ferrum_state::change::MapMode` | enum | `ferrum/crates/ferrum-state/src/change.rs:6` | C2 |
| state.fe.003 | `ferrum_state::change::MapMode::Simple` | variant | `ferrum/crates/ferrum-state/src/change.rs:8` | C2 |
| state.fe.004 | `ferrum_state::change::MapMode::TrackDel` | variant | `ferrum/crates/ferrum-state/src/change.rs:10` | C2 |
| state.fe.005 | `ferrum_state::change::MapMode::TrackBefore` | variant | `ferrum/crates/ferrum-state/src/change.rs:12` | C2 |
| state.fe.006 | `ferrum_state::change::MapMode::TrackAfter` | variant | `ferrum/crates/ferrum-state/src/change.rs:14` | C2 |
| state.fe.007 | `ferrum_state::change::ChangeDesc` | struct | `ferrum/crates/ferrum-state/src/change.rs:22` | C2 |
| state.fe.008 | `ferrum_state::change::ChangeDesc::create` | method | `ferrum/crates/ferrum-state/src/change.rs:28` | C2 |
| state.fe.009 | `ferrum_state::change::ChangeDesc::empty` | method | `ferrum/crates/ferrum-state/src/change.rs:33` | C2 |
| state.fe.010 | `ferrum_state::change::ChangeDesc::length` | method | `ferrum/crates/ferrum-state/src/change.rs:46` | C2 |
| state.fe.011 | `ferrum_state::change::ChangeDesc::new_length` | method | `ferrum/crates/ferrum-state/src/change.rs:57` | C2 |
| state.fe.012 | `ferrum_state::change::ChangeDesc::is_empty` | method | `ferrum/crates/ferrum-state/src/change.rs:73` | C2 |
| state.fe.013 | `ferrum_state::change::ChangeDesc::iter_gaps` | method | `ferrum/crates/ferrum-state/src/change.rs:79` | C2 |
| state.fe.014 | `ferrum_state::change::ChangeDesc::iter_changed_ranges` | method | `ferrum/crates/ferrum-state/src/change.rs:99` | C2 |
| state.fe.015 | `ferrum_state::change::ChangeDesc::inverted_desc` | method | `ferrum/crates/ferrum-state/src/change.rs:106` | C2 |
| state.fe.016 | `ferrum_state::change::ChangeDesc::compose_desc` | method | `ferrum/crates/ferrum-state/src/change.rs:126` | C2 |
| state.fe.017 | `ferrum_state::change::ChangeDesc::map_desc` | method | `ferrum/crates/ferrum-state/src/change.rs:138` | C2 |
| state.fe.018 | `ferrum_state::change::ChangeDesc::map_pos` | method | `ferrum/crates/ferrum-state/src/change.rs:149` | C2 |
| state.fe.019 | `ferrum_state::change::ChangeDesc::from_string` | method | `ferrum/crates/ferrum-state/src/change.rs:198` | C2 |
| state.fe.020 | `ferrum_state::change::ChangeSpec` | struct | `ferrum/crates/ferrum-state/src/change.rs:246` | C2 |
| state.fe.021 | `ferrum_state::change::ChangeSpec.from` | field | `ferrum/crates/ferrum-state/src/change.rs:247` | C2 |
| state.fe.022 | `ferrum_state::change::ChangeSpec.to` | field | `ferrum/crates/ferrum-state/src/change.rs:248` | C2 |
| state.fe.023 | `ferrum_state::change::ChangeSpec.insert` | field | `ferrum/crates/ferrum-state/src/change.rs:249` | C2 |
| state.fe.024 | `ferrum_state::change::ChangeSpec::delete` | method | `ferrum/crates/ferrum-state/src/change.rs:253` | C2 |
| state.fe.025 | `ferrum_state::change::ChangeSpec::insert` | method | `ferrum/crates/ferrum-state/src/change.rs:261` | C2 |
| state.fe.026 | `ferrum_state::change::ChangeSpec::replace` | method | `ferrum/crates/ferrum-state/src/change.rs:269` | C2 |
| state.fe.027 | `ferrum_state::change::Changes` | enum | `ferrum/crates/ferrum-state/src/change.rs:280` | C2 |
| state.fe.028 | `ferrum_state::change::Changes::Set` | variant | `ferrum/crates/ferrum-state/src/change.rs:282` | C2 |
| state.fe.029 | `ferrum_state::change::Changes::Single` | variant | `ferrum/crates/ferrum-state/src/change.rs:284` | C2 |
| state.fe.030 | `ferrum_state::change::Changes::Multiple` | variant | `ferrum/crates/ferrum-state/src/change.rs:286` | C2 |
| state.fe.031 | `ferrum_state::change::Changes::to_changeset` | method | `ferrum/crates/ferrum-state/src/change.rs:291` | C2 |
| state.fe.032 | `ferrum_state::change::ChangeSet` | struct | `ferrum/crates/ferrum-state/src/change.rs:320` | C2 |
| state.fe.033 | `ferrum_state::change::ChangeSet::create_set` | method | `ferrum/crates/ferrum-state/src/change.rs:327` | C2 |
| state.fe.034 | `ferrum_state::change::ChangeSet::empty` | method | `ferrum/crates/ferrum-state/src/change.rs:335` | C2 |
| state.fe.035 | `ferrum_state::change::ChangeSet::length` | method | `ferrum/crates/ferrum-state/src/change.rs:343` | C2 |
| state.fe.036 | `ferrum_state::change::ChangeSet::new_length` | method | `ferrum/crates/ferrum-state/src/change.rs:348` | C2 |
| state.fe.037 | `ferrum_state::change::ChangeSet::is_empty` | method | `ferrum/crates/ferrum-state/src/change.rs:353` | C2 |
| state.fe.038 | `ferrum_state::change::ChangeSet::desc` | method | `ferrum/crates/ferrum-state/src/change.rs:358` | C2 |
| state.fe.039 | `ferrum_state::change::ChangeSet::map_pos` | method | `ferrum/crates/ferrum-state/src/change.rs:363` | C2 |
| state.fe.040 | `ferrum_state::change::ChangeSet::iter_changes` | method | `ferrum/crates/ferrum-state/src/change.rs:368` | C2 |
| state.fe.041 | `ferrum_state::change::ChangeSet::iter_gaps` | method | `ferrum/crates/ferrum-state/src/change.rs:374` | C2 |
| state.fe.042 | `ferrum_state::change::ChangeSet::apply` | method | `ferrum/crates/ferrum-state/src/change.rs:379` | C2 |
| state.fe.043 | `ferrum_state::change::ChangeSet::invert` | method | `ferrum/crates/ferrum-state/src/change.rs:412` | C2 |
| state.fe.044 | `ferrum_state::change::ChangeSet::compose` | method | `ferrum/crates/ferrum-state/src/change.rs:446` | C2 |
| state.fe.045 | `ferrum_state::change::ChangeSet::map` | method | `ferrum/crates/ferrum-state/src/change.rs:466` | C2 |
| state.fe.046 | `ferrum_state::change::ChangeSet::of_spec` | method | `ferrum/crates/ferrum-state/src/change.rs:479` | C2 |
| state.fe.047 | `ferrum_state::change::ChangeSet::of_specs` | method | `ferrum/crates/ferrum-state/src/change.rs:484` | C2 |
| state.fe.048 | `ferrum_state::char` | module | `ferrum/crates/ferrum-state/src/char.rs:1` | main |
| state.fe.049 | `ferrum_state::char::CharCategory` | enum | `ferrum/crates/ferrum-state/src/char.rs:5` | C3 |
| state.fe.050 | `ferrum_state::char::CharCategory::Word` | variant | `ferrum/crates/ferrum-state/src/char.rs:6` | C3 |
| state.fe.051 | `ferrum_state::char::CharCategory::Space` | variant | `ferrum/crates/ferrum-state/src/char.rs:7` | C3 |
| state.fe.052 | `ferrum_state::char::CharCategory::Other` | variant | `ferrum/crates/ferrum-state/src/char.rs:8` | C3 |
| state.fe.053 | `ferrum_state::char::categorize_char` | function | `ferrum/crates/ferrum-state/src/char.rs:12` | C3 |
| state.fe.054 | `ferrum_state::char::find_cluster_break` | function | `ferrum/crates/ferrum-state/src/char.rs:31` | C3 |
| state.fe.055 | `ferrum_state::char::find_word_break` | function | `ferrum/crates/ferrum-state/src/char.rs:70` | C3 |
| state.fe.056 | `ferrum_state::char::find_subword_break` | function | `ferrum/crates/ferrum-state/src/char.rs:101` | C3 |
| state.fe.057 | `ferrum_state::column` | module | `ferrum/crates/ferrum-state/src/column.rs:1` | main |
| state.fe.058 | `ferrum_state::column::count_column` | function | `ferrum/crates/ferrum-state/src/column.rs:8` | C3 |
| state.fe.059 | `ferrum_state::column::find_column` | function | `ferrum/crates/ferrum-state/src/column.rs:28` | C3 |
| state.fe.060 | `ferrum_state::config` | module | `ferrum/crates/ferrum-state/src/config.rs:1` | main |
| state.fe.061 | `ferrum_state::config::Configuration` | struct | `ferrum/crates/ferrum-state/src/config.rs:24` | C4 |
| state.fe.062 | `ferrum_state::config::Configuration::resolve` | method | `ferrum/crates/ferrum-state/src/config.rs:36` | C4 |
| state.fe.063 | `ferrum_state::config::Configuration::facet` | method | `ferrum/crates/ferrum-state/src/config.rs:168` | C4 |
| state.fe.064 | `ferrum_state::config::Configuration::field` | method | `ferrum/crates/ferrum-state/src/config.rs:192` | C4 |
| state.fe.065 | `ferrum_state::facet` | module | `ferrum/crates/ferrum-state/src/facet.rs:1` | main |
| state.fe.066 | `ferrum_state::facet::FacetId` | struct | `ferrum/crates/ferrum-state/src/facet.rs:11` | C4 |
| state.fe.067 | `ferrum_state::facet::StateFieldId` | struct | `ferrum/crates/ferrum-state/src/facet.rs:15` | C4 |
| state.fe.068 | `ferrum_state::facet::CompartmentId` | struct | `ferrum/crates/ferrum-state/src/facet.rs:19` | C4 |
| state.fe.069 | `ferrum_state::facet::Prec` | enum | `ferrum/crates/ferrum-state/src/facet.rs:23` | C4 |
| state.fe.070 | `ferrum_state::facet::Prec::Highest` | variant | `ferrum/crates/ferrum-state/src/facet.rs:24` | C4 |
| state.fe.071 | `ferrum_state::facet::Prec::High` | variant | `ferrum/crates/ferrum-state/src/facet.rs:25` | C4 |
| state.fe.072 | `ferrum_state::facet::Prec::Default` | variant | `ferrum/crates/ferrum-state/src/facet.rs:26` | C4 |
| state.fe.073 | `ferrum_state::facet::Prec::Low` | variant | `ferrum/crates/ferrum-state/src/facet.rs:27` | C4 |
| state.fe.074 | `ferrum_state::facet::Prec::Lowest` | variant | `ferrum/crates/ferrum-state/src/facet.rs:28` | C4 |
| state.fe.075 | `ferrum_state::facet::AnyFacet` | trait | `ferrum/crates/ferrum-state/src/facet.rs:36` | C4 |
| state.fe.076 | `ferrum_state::facet::AnyFacet::id` | trait method | `ferrum/crates/ferrum-state/src/facet.rs:37` | C4 |
| state.fe.077 | `ferrum_state::facet::AnyFacet::is_static` | trait method | `ferrum/crates/ferrum-state/src/facet.rs:38` | C4 |
| state.fe.078 | `ferrum_state::facet::AnyFacet::combine_erased` | trait method | `ferrum/crates/ferrum-state/src/facet.rs:39` | C4 |
| state.fe.079 | `ferrum_state::facet::AnyFacet::default_erased` | trait method | `ferrum/crates/ferrum-state/src/facet.rs:40` | C4 |
| state.fe.080 | `ferrum_state::facet::AnyStateField` | trait | `ferrum/crates/ferrum-state/src/facet.rs:44` | C4 |
| state.fe.081 | `ferrum_state::facet::AnyStateField::id` | trait method | `ferrum/crates/ferrum-state/src/facet.rs:45` | C4 |
| state.fe.082 | `ferrum_state::facet::AnyStateField::init` | trait method | `ferrum/crates/ferrum-state/src/facet.rs:46` | C4 |
| state.fe.083 | `ferrum_state::facet::AnyStateField::update` | trait method | `ferrum/crates/ferrum-state/src/facet.rs:47` | C4 |
| state.fe.084 | `ferrum_state::facet::Facet` | struct | `ferrum/crates/ferrum-state/src/facet.rs:53` | C4 |
| state.fe.085 | `ferrum_state::facet::Facet::define` | method | `ferrum/crates/ferrum-state/src/facet.rs:64` | C4 |
| state.fe.086 | `ferrum_state::facet::Facet::define_static` | method | `ferrum/crates/ferrum-state/src/facet.rs:78` | C4 |
| state.fe.087 | `ferrum_state::facet::Facet::id` | method | `ferrum/crates/ferrum-state/src/facet.rs:88` | C4 |
| state.fe.088 | `ferrum_state::facet::Facet::default_value` | method | `ferrum/crates/ferrum-state/src/facet.rs:93` | C4 |
| state.fe.089 | `ferrum_state::facet::Facet::of` | method | `ferrum/crates/ferrum-state/src/facet.rs:98` | C4 |
| state.fe.090 | `ferrum_state::facet::Facet::compute` | method | `ferrum/crates/ferrum-state/src/facet.rs:103` | C4 |
| state.fe.091 | `ferrum_state::facet::Facet::compute_multi` | method | `ferrum/crates/ferrum-state/src/facet.rs:115` | C4 |
| state.fe.092 | `ferrum_state::facet::Facet::define_list` | method | `ferrum/crates/ferrum-state/src/facet.rs:132` | C4 |
| state.fe.093 | `ferrum_state::facet::Facet as AnyFacet::id` | trait impl method | `ferrum/crates/ferrum-state/src/facet.rs:140` | C4 |
| state.fe.094 | `ferrum_state::facet::Facet as AnyFacet::is_static` | trait impl method | `ferrum/crates/ferrum-state/src/facet.rs:144` | C4 |
| state.fe.095 | `ferrum_state::facet::Facet as AnyFacet::combine_erased` | trait impl method | `ferrum/crates/ferrum-state/src/facet.rs:148` | C4 |
| state.fe.096 | `ferrum_state::facet::Facet as AnyFacet::default_erased` | trait impl method | `ferrum/crates/ferrum-state/src/facet.rs:161` | C4 |
| state.fe.097 | `ferrum_state::facet::StateField` | struct | `ferrum/crates/ferrum-state/src/facet.rs:168` | C4 |
| state.fe.098 | `ferrum_state::facet::StateField::define` | method | `ferrum/crates/ferrum-state/src/facet.rs:176` | C4 |
| state.fe.099 | `ferrum_state::facet::StateField::id` | method | `ferrum/crates/ferrum-state/src/facet.rs:189` | C4 |
| state.fe.100 | `ferrum_state::facet::StateField::extension` | method | `ferrum/crates/ferrum-state/src/facet.rs:194` | C4 |
| state.fe.101 | `ferrum_state::facet::StateField as AnyStateField::id` | trait impl method | `ferrum/crates/ferrum-state/src/facet.rs:200` | C4 |
| state.fe.102 | `ferrum_state::facet::StateField as AnyStateField::init` | trait impl method | `ferrum/crates/ferrum-state/src/facet.rs:204` | C4 |
| state.fe.103 | `ferrum_state::facet::StateField as AnyStateField::update` | trait impl method | `ferrum/crates/ferrum-state/src/facet.rs:208` | C4 |
| state.fe.104 | `ferrum_state::facet::Compartment` | struct | `ferrum/crates/ferrum-state/src/facet.rs:218` | C4 |
| state.fe.105 | `ferrum_state::facet::Compartment.id` | field | `ferrum/crates/ferrum-state/src/facet.rs:219` | C4 |
| state.fe.106 | `ferrum_state::facet::Compartment::new` | method | `ferrum/crates/ferrum-state/src/facet.rs:230` | C4 |
| state.fe.107 | `ferrum_state::facet::Compartment::of` | method | `ferrum/crates/ferrum-state/src/facet.rs:237` | C4 |
| state.fe.108 | `ferrum_state::facet::Compartment::reconfigure` | method | `ferrum/crates/ferrum-state/src/transaction.rs:57` | C4 |
| state.fe.109 | `ferrum_state::facet::Extension` | enum | `ferrum/crates/ferrum-state/src/facet.rs:244` | C4 |
| state.fe.110 | `ferrum_state::facet::Extension::FacetStatic` | variant | `ferrum/crates/ferrum-state/src/facet.rs:246` | C4 |
| state.fe.111 | `ferrum_state::facet::Extension::FacetDynamic` | variant | `ferrum/crates/ferrum-state/src/facet.rs:248` | C4 |
| state.fe.112 | `ferrum_state::facet::Extension::FacetDynamicMulti` | variant | `ferrum/crates/ferrum-state/src/facet.rs:253` | C4 |
| state.fe.113 | `ferrum_state::facet::Extension::StateField` | variant | `ferrum/crates/ferrum-state/src/facet.rs:258` | C4 |
| state.fe.114 | `ferrum_state::facet::Extension::Precedence` | variant | `ferrum/crates/ferrum-state/src/facet.rs:260` | C4 |
| state.fe.115 | `ferrum_state::facet::Extension::Compartment` | variant | `ferrum/crates/ferrum-state/src/facet.rs:262` | C4 |
| state.fe.116 | `ferrum_state::facet::Extension::Group` | variant | `ferrum/crates/ferrum-state/src/facet.rs:264` | C4 |
| state.fe.117 | `ferrum_state::facet::Extension::with_prec` | method | `ferrum/crates/ferrum-state/src/facet.rs:269` | C4 |
| state.fe.118 | `ferrum_state::rangeset` | module | `ferrum/crates/ferrum-state/src/rangeset.rs:1` | main |
| state.fe.119 | `ferrum_state::rangeset::CHUNK_SIZE` | constant | `ferrum/crates/ferrum-state/src/rangeset.rs:10` | C6 |
| state.fe.120 | `ferrum_state::rangeset::RangeValue` | trait | `ferrum/crates/ferrum-state/src/rangeset.rs:14` | C6 |
| state.fe.121 | `ferrum_state::rangeset::RangeValue::start_side` | trait method (provided) | `ferrum/crates/ferrum-state/src/rangeset.rs:16` | C6 |
| state.fe.122 | `ferrum_state::rangeset::RangeValue::end_side` | trait method (provided) | `ferrum/crates/ferrum-state/src/rangeset.rs:18` | C6 |
| state.fe.123 | `ferrum_state::rangeset::RangeValue::point` | trait method (provided) | `ferrum/crates/ferrum-state/src/rangeset.rs:20` | C6 |
| state.fe.124 | `ferrum_state::rangeset::RangeValue::map_mode` | trait method (provided) | `ferrum/crates/ferrum-state/src/rangeset.rs:22` | C6 |
| state.fe.125 | `ferrum_state::rangeset::RangeValue::eq_val` | trait method (provided) | `ferrum/crates/ferrum-state/src/rangeset.rs:24` | C6 |
| state.fe.126 | `ferrum_state::rangeset::Range` | struct | `ferrum/crates/ferrum-state/src/rangeset.rs:29` | C6 |
| state.fe.127 | `ferrum_state::rangeset::Range.from` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:30` | C6 |
| state.fe.128 | `ferrum_state::rangeset::Range.to` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:31` | C6 |
| state.fe.129 | `ferrum_state::rangeset::Range.value` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:32` | C6 |
| state.fe.130 | `ferrum_state::rangeset::Range::new` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:37` | C6 |
| state.fe.131 | `ferrum_state::rangeset::Chunk` | struct | `ferrum/crates/ferrum-state/src/rangeset.rs:44` | C6 |
| state.fe.132 | `ferrum_state::rangeset::Chunk.from` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:45` | C6 |
| state.fe.133 | `ferrum_state::rangeset::Chunk.to` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:46` | C6 |
| state.fe.134 | `ferrum_state::rangeset::Chunk.value` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:47` | C6 |
| state.fe.135 | `ferrum_state::rangeset::Chunk.max_point` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:48` | C6 |
| state.fe.136 | `ferrum_state::rangeset::Chunk::new` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:52` | C6 |
| state.fe.137 | `ferrum_state::rangeset::Chunk::length` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:57` | C6 |
| state.fe.138 | `ferrum_state::rangeset::Chunk::find_index` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:61` | C6 |
| state.fe.139 | `ferrum_state::rangeset::Chunk::between` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:78` | C6 |
| state.fe.140 | `ferrum_state::rangeset::RangeSet` | struct | `ferrum/crates/ferrum-state/src/rangeset.rs:92` | C6 |
| state.fe.141 | `ferrum_state::rangeset::RangeSet.chunk_pos` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:93` | C6 |
| state.fe.142 | `ferrum_state::rangeset::RangeSet.chunk` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:94` | C6 |
| state.fe.143 | `ferrum_state::rangeset::RangeSet.next_layer` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:95` | C6 |
| state.fe.144 | `ferrum_state::rangeset::RangeSet.max_point` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:96` | C6 |
| state.fe.145 | `ferrum_state::rangeset::RangeSet::empty` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:107` | C6 |
| state.fe.146 | `ferrum_state::rangeset::RangeSet::is_empty` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:117` | C6 |
| state.fe.147 | `ferrum_state::rangeset::RangeSet::size` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:122` | C6 |
| state.fe.148 | `ferrum_state::rangeset::RangeSet::length` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:134` | C6 |
| state.fe.149 | `ferrum_state::rangeset::RangeSet::of` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:148` | C6 |
| state.fe.150 | `ferrum_state::rangeset::RangeSet::of_sorted` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:153` | C6 |
| state.fe.151 | `ferrum_state::rangeset::RangeSet::between` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:168` | C6 |
| state.fe.152 | `ferrum_state::rangeset::RangeSet::iter` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:191` | C6 |
| state.fe.153 | `ferrum_state::rangeset::RangeSet::update` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:218` | C6 |
| state.fe.154 | `ferrum_state::rangeset::RangeSet::map` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:239` | C6 |
| state.fe.155 | `ferrum_state::rangeset::RangeSet::compare` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:896` | C6 |
| state.fe.156 | `ferrum_state::rangeset::RangeSet::spans` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:926` | C6 |
| state.fe.157 | `ferrum_state::rangeset::RangeSetUpdate` | struct | `ferrum/crates/ferrum-state/src/rangeset.rs:274` | C6 |
| state.fe.158 | `ferrum_state::rangeset::RangeSetUpdate.add` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:275` | C6 |
| state.fe.159 | `ferrum_state::rangeset::RangeSetUpdate.sort` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:276` | C6 |
| state.fe.160 | `ferrum_state::rangeset::RangeSetUpdate.filter` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:277` | C6 |
| state.fe.161 | `ferrum_state::rangeset::RangeSetUpdate.filter_from` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:278` | C6 |
| state.fe.162 | `ferrum_state::rangeset::RangeSetUpdate.filter_to` | field | `ferrum/crates/ferrum-state/src/rangeset.rs:279` | C6 |
| state.fe.163 | `ferrum_state::rangeset::RangeSetBuilder` | struct | `ferrum/crates/ferrum-state/src/rangeset.rs:295` | C6 |
| state.fe.164 | `ferrum_state::rangeset::RangeSetBuilder::new` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:319` | C6 |
| state.fe.165 | `ferrum_state::rangeset::RangeSetBuilder::add` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:353` | C6 |
| state.fe.166 | `ferrum_state::rangeset::RangeSetBuilder::add_strict` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:358` | C6 |
| state.fe.167 | `ferrum_state::rangeset::RangeSetBuilder::add_range` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:362` | C6 |
| state.fe.168 | `ferrum_state::rangeset::RangeSetBuilder::finish` | method | `ferrum/crates/ferrum-state/src/rangeset.rs:405` | C6 |
| state.fe.169 | `ferrum_state::rangeset::RangeComparator` | trait | `ferrum/crates/ferrum-state/src/rangeset.rs:418` | C6 |
| state.fe.170 | `ferrum_state::rangeset::RangeComparator::compare_range` | trait method | `ferrum/crates/ferrum-state/src/rangeset.rs:419` | C6 |
| state.fe.171 | `ferrum_state::rangeset::RangeComparator::compare_point` | trait method | `ferrum/crates/ferrum-state/src/rangeset.rs:420` | C6 |
| state.fe.172 | `ferrum_state::rangeset::SpanIterator` | trait | `ferrum/crates/ferrum-state/src/rangeset.rs:424` | C6 |
| state.fe.173 | `ferrum_state::rangeset::SpanIterator::span` | trait method | `ferrum/crates/ferrum-state/src/rangeset.rs:425` | C6 |
| state.fe.174 | `ferrum_state::rangeset::SpanIterator::point` | trait method | `ferrum/crates/ferrum-state/src/rangeset.rs:426` | C6 |
| state.fe.175 | `ferrum_state::selection` | module | `ferrum/crates/ferrum-state/src/selection.rs:1` | main |
| state.fe.176 | `ferrum_state::selection::SelectionRange` | struct | `ferrum/crates/ferrum-state/src/selection.rs:7` | C3 |
| state.fe.177 | `ferrum_state::selection::SelectionRange.anchor` | field | `ferrum/crates/ferrum-state/src/selection.rs:9` | C3 |
| state.fe.178 | `ferrum_state::selection::SelectionRange.head` | field | `ferrum/crates/ferrum-state/src/selection.rs:11` | C3 |
| state.fe.179 | `ferrum_state::selection::SelectionRange.goal_column` | field | `ferrum/crates/ferrum-state/src/selection.rs:13` | C3 |
| state.fe.180 | `ferrum_state::selection::SelectionRange.assoc` | field | `ferrum/crates/ferrum-state/src/selection.rs:15` | C3 |
| state.fe.181 | `ferrum_state::selection::SelectionRange::cursor` | method | `ferrum/crates/ferrum-state/src/selection.rs:20` | C3 |
| state.fe.182 | `ferrum_state::selection::SelectionRange::new` | method | `ferrum/crates/ferrum-state/src/selection.rs:30` | C3 |
| state.fe.183 | `ferrum_state::selection::SelectionRange::from` | method | `ferrum/crates/ferrum-state/src/selection.rs:41` | C3 |
| state.fe.184 | `ferrum_state::selection::SelectionRange::to` | method | `ferrum/crates/ferrum-state/src/selection.rs:47` | C3 |
| state.fe.185 | `ferrum_state::selection::SelectionRange::is_empty` | method | `ferrum/crates/ferrum-state/src/selection.rs:53` | C3 |
| state.fe.186 | `ferrum_state::selection::SelectionRange::is_inverted` | method | `ferrum/crates/ferrum-state/src/selection.rs:59` | C3 |
| state.fe.187 | `ferrum_state::selection::SelectionRange::with_goal_column` | method | `ferrum/crates/ferrum-state/src/selection.rs:64` | C3 |
| state.fe.188 | `ferrum_state::selection::SelectionRange::map` | method | `ferrum/crates/ferrum-state/src/selection.rs:70` | C3 |
| state.fe.189 | `ferrum_state::selection::SelectionRange::extend` | method | `ferrum/crates/ferrum-state/src/selection.rs:98` | C3 |
| state.fe.190 | `ferrum_state::selection::EditorSelection` | struct | `ferrum/crates/ferrum-state/src/selection.rs:117` | C3 |
| state.fe.191 | `ferrum_state::selection::EditorSelection::cursor` | method | `ferrum/crates/ferrum-state/src/selection.rs:130` | C3 |
| state.fe.192 | `ferrum_state::selection::EditorSelection::single` | method | `ferrum/crates/ferrum-state/src/selection.rs:138` | C3 |
| state.fe.193 | `ferrum_state::selection::EditorSelection::create` | method | `ferrum/crates/ferrum-state/src/selection.rs:147` | C3 |
| state.fe.194 | `ferrum_state::selection::EditorSelection::main` | method | `ferrum/crates/ferrum-state/src/selection.rs:207` | C3 |
| state.fe.195 | `ferrum_state::selection::EditorSelection::main_index` | method | `ferrum/crates/ferrum-state/src/selection.rs:213` | C3 |
| state.fe.196 | `ferrum_state::selection::EditorSelection::ranges` | method | `ferrum/crates/ferrum-state/src/selection.rs:219` | C3 |
| state.fe.197 | `ferrum_state::selection::EditorSelection::is_single` | method | `ferrum/crates/ferrum-state/src/selection.rs:225` | C3 |
| state.fe.198 | `ferrum_state::selection::EditorSelection::map` | method | `ferrum/crates/ferrum-state/src/selection.rs:230` | C3 |
| state.fe.199 | `ferrum_state::state` | module | `ferrum/crates/ferrum-state/src/state.rs:1` | main |
| state.fe.200 | `ferrum_state::state::RangeResult` | struct | `ferrum/crates/ferrum-state/src/state.rs:31` | C5 |
| state.fe.201 | `ferrum_state::state::RangeResult.range` | field | `ferrum/crates/ferrum-state/src/state.rs:32` | C5 |
| state.fe.202 | `ferrum_state::state::RangeResult.changes` | field | `ferrum/crates/ferrum-state/src/state.rs:33` | C5 |
| state.fe.203 | `ferrum_state::state::RangeResult.effects` | field | `ferrum/crates/ferrum-state/src/state.rs:34` | C5 |
| state.fe.204 | `ferrum_state::state::RangeResult::range` | method | `ferrum/crates/ferrum-state/src/state.rs:39` | C5 |
| state.fe.205 | `ferrum_state::state::RangeResult::with_changes` | method | `ferrum/crates/ferrum-state/src/state.rs:48` | C5 |
| state.fe.206 | `ferrum_state::state::RangeResult::with_effect` | method | `ferrum/crates/ferrum-state/src/state.rs:54` | C5 |
| state.fe.207 | `ferrum_state::state::ChangeByRangeResult` | struct | `ferrum/crates/ferrum-state/src/state.rs:62` | C5 |
| state.fe.208 | `ferrum_state::state::ChangeByRangeResult.changes` | field | `ferrum/crates/ferrum-state/src/state.rs:63` | C5 |
| state.fe.209 | `ferrum_state::state::ChangeByRangeResult.selection` | field | `ferrum/crates/ferrum-state/src/state.rs:64` | C5 |
| state.fe.210 | `ferrum_state::state::ChangeByRangeResult.effects` | field | `ferrum/crates/ferrum-state/src/state.rs:65` | C5 |
| state.fe.211 | `ferrum_state::state::ChangeByRangeResult::into_spec` | method | `ferrum/crates/ferrum-state/src/state.rs:70` | C5 |
| state.fe.212 | `ferrum_state::state::EditorStateConfig` | struct | `ferrum/crates/ferrum-state/src/state.rs:83` | C5 |
| state.fe.213 | `ferrum_state::state::EditorStateConfig.doc` | field | `ferrum/crates/ferrum-state/src/state.rs:85` | C5 |
| state.fe.214 | `ferrum_state::state::EditorStateConfig.selection` | field | `ferrum/crates/ferrum-state/src/state.rs:87` | C5 |
| state.fe.215 | `ferrum_state::state::EditorStateConfig.extensions` | field | `ferrum/crates/ferrum-state/src/state.rs:89` | C5 |
| state.fe.216 | `ferrum_state::state::EditorState` | struct | `ferrum/crates/ferrum-state/src/state.rs:95` | C5 |
| state.fe.217 | `ferrum_state::state::EditorState::create` | method | `ferrum/crates/ferrum-state/src/state.rs:105` | C5 |
| state.fe.218 | `ferrum_state::state::EditorState::doc` | method | `ferrum/crates/ferrum-state/src/state.rs:138` | C5 |
| state.fe.219 | `ferrum_state::state::EditorState::selection` | method | `ferrum/crates/ferrum-state/src/state.rs:143` | C5 |
| state.fe.220 | `ferrum_state::state::EditorState::facet` | method | `ferrum/crates/ferrum-state/src/state.rs:149` | C5 |
| state.fe.221 | `ferrum_state::state::EditorState::field` | method | `ferrum/crates/ferrum-state/src/state.rs:157` | C5 |
| state.fe.222 | `ferrum_state::state::EditorState::allow_multiple_selections` | method | `ferrum/crates/ferrum-state/src/state.rs:165` | C5 |
| state.fe.223 | `ferrum_state::state::EditorState::tab_size` | method | `ferrum/crates/ferrum-state/src/state.rs:170` | C5 |
| state.fe.224 | `ferrum_state::state::EditorState::read_only` | method | `ferrum/crates/ferrum-state/src/state.rs:175` | C5 |
| state.fe.225 | `ferrum_state::state::EditorState::is_read_only` | method | `ferrum/crates/ferrum-state/src/state.rs:180` | C5 |
| state.fe.226 | `ferrum_state::state::EditorState::line_separator` | method | `ferrum/crates/ferrum-state/src/state.rs:185` | C5 |
| state.fe.227 | `ferrum_state::state::EditorState::char_categorizer` | method | `ferrum/crates/ferrum-state/src/state.rs:190` | C5 |
| state.fe.228 | `ferrum_state::state::EditorState::changes` | method | `ferrum/crates/ferrum-state/src/state.rs:201` | C5 |
| state.fe.229 | `ferrum_state::state::EditorState::change_by_range` | method | `ferrum/crates/ferrum-state/src/state.rs:208` | C5 |
| state.fe.230 | `ferrum_state::state::EditorState::replace_selection` | method | `ferrum/crates/ferrum-state/src/state.rs:245` | C5 |
| state.fe.231 | `ferrum_state::state::EditorState::update` | method | `ferrum/crates/ferrum-state/src/state.rs:257` | C5 |
| state.fe.232 | `ferrum_state::state::EditorState::apply` | method | `ferrum/crates/ferrum-state/src/state.rs:308` | C5 |
| state.fe.233 | `ferrum_state::text` | module | `ferrum/crates/ferrum-state/src/text.rs:1` | main |
| state.fe.234 | `ferrum_state::text::Text` | struct | `ferrum/crates/ferrum-state/src/text.rs:9` | C1 |
| state.fe.235 | `ferrum_state::text::Text::empty` | method | `ferrum/crates/ferrum-state/src/text.rs:21` | C1 |
| state.fe.236 | `ferrum_state::text::Text::from_str` | method | `ferrum/crates/ferrum-state/src/text.rs:28` | C1 |
| state.fe.237 | `ferrum_state::text::Text::len` | method | `ferrum/crates/ferrum-state/src/text.rs:36` | C1 |
| state.fe.238 | `ferrum_state::text::Text::len_chars` | method | `ferrum/crates/ferrum-state/src/text.rs:42` | C1 |
| state.fe.239 | `ferrum_state::text::Text::len_bytes` | method | `ferrum/crates/ferrum-state/src/text.rs:47` | C1 |
| state.fe.240 | `ferrum_state::text::Text::len_lines` | method | `ferrum/crates/ferrum-state/src/text.rs:52` | C1 |
| state.fe.241 | `ferrum_state::text::Text::is_empty` | method | `ferrum/crates/ferrum-state/src/text.rs:57` | C1 |
| state.fe.242 | `ferrum_state::text::Text::char_to_line` | method | `ferrum/crates/ferrum-state/src/text.rs:62` | C1 |
| state.fe.243 | `ferrum_state::text::Text::line_to_char` | method | `ferrum/crates/ferrum-state/src/text.rs:68` | C1 |
| state.fe.244 | `ferrum_state::text::Text::char_to_byte` | method | `ferrum/crates/ferrum-state/src/text.rs:74` | C1 |
| state.fe.245 | `ferrum_state::text::Text::byte_to_char` | method | `ferrum/crates/ferrum-state/src/text.rs:80` | C1 |
| state.fe.246 | `ferrum_state::text::Text::of` | method | `ferrum/crates/ferrum-state/src/text.rs:86` | C1 |
| state.fe.247 | `ferrum_state::text::Text::line` | method | `ferrum/crates/ferrum-state/src/text.rs:97` | C1 |
| state.fe.248 | `ferrum_state::text::Text::line_by_index` | method | `ferrum/crates/ferrum-state/src/text.rs:105` | C1 |
| state.fe.249 | `ferrum_state::text::Text::line_at` | method | `ferrum/crates/ferrum-state/src/text.rs:133` | C1 |
| state.fe.250 | `ferrum_state::text::Text::slice_to_string` | method | `ferrum/crates/ferrum-state/src/text.rs:142` | C1 |
| state.fe.251 | `ferrum_state::text::Text::slice_string` | method | `ferrum/crates/ferrum-state/src/text.rs:149` | C1 |
| state.fe.252 | `ferrum_state::text::Text::replace` | method | `ferrum/crates/ferrum-state/src/text.rs:154` | C1 |
| state.fe.253 | `ferrum_state::text::Text::slice` | method | `ferrum/crates/ferrum-state/src/text.rs:170` | C1 |
| state.fe.254 | `ferrum_state::text::Text::append` | method | `ferrum/crates/ferrum-state/src/text.rs:182` | C1 |
| state.fe.255 | `ferrum_state::text::Text::as_rope` | method | `ferrum/crates/ferrum-state/src/text.rs:195` | C1 |
| state.fe.256 | `ferrum_state::text::Line` | struct | `ferrum/crates/ferrum-state/src/text.rs:232` | C1 |
| state.fe.257 | `ferrum_state::text::Line.number` | field | `ferrum/crates/ferrum-state/src/text.rs:234` | C1 |
| state.fe.258 | `ferrum_state::text::Line.from` | field | `ferrum/crates/ferrum-state/src/text.rs:236` | C1 |
| state.fe.259 | `ferrum_state::text::Line.to` | field | `ferrum/crates/ferrum-state/src/text.rs:238` | C1 |
| state.fe.260 | `ferrum_state::text::Line.length` | field | `ferrum/crates/ferrum-state/src/text.rs:240` | C1 |
| state.fe.261 | `ferrum_state::text::Line.text` | field | `ferrum/crates/ferrum-state/src/text.rs:242` | C1 |
| state.fe.262 | `ferrum_state::transaction` | module | `ferrum/crates/ferrum-state/src/transaction.rs:1` | main |
| state.fe.263 | `ferrum_state::transaction::StateEffect` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:11` | C5 |
| state.fe.264 | `ferrum_state::transaction::StateEffect::of` | method | `ferrum/crates/ferrum-state/src/transaction.rs:26` | C5 |
| state.fe.265 | `ferrum_state::transaction::StateEffect::is` | method | `ferrum/crates/ferrum-state/src/transaction.rs:34` | C5 |
| state.fe.266 | `ferrum_state::transaction::StateEffect::downcast_ref` | method | `ferrum/crates/ferrum-state/src/transaction.rs:39` | C5 |
| state.fe.267 | `ferrum_state::transaction::CompartmentReconfigure` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:50` | C5 |
| state.fe.268 | `ferrum_state::transaction::CompartmentReconfigure.compartment` | field | `ferrum/crates/ferrum-state/src/transaction.rs:51` | C5 |
| state.fe.269 | `ferrum_state::transaction::CompartmentReconfigure.extension` | field | `ferrum/crates/ferrum-state/src/transaction.rs:52` | C5 |
| state.fe.270 | `ferrum_state::transaction::TransactionSpec` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:67` | C5 |
| state.fe.271 | `ferrum_state::transaction::TransactionSpec.changes` | field | `ferrum/crates/ferrum-state/src/transaction.rs:69` | C5 |
| state.fe.272 | `ferrum_state::transaction::TransactionSpec.selection` | field | `ferrum/crates/ferrum-state/src/transaction.rs:71` | C5 |
| state.fe.273 | `ferrum_state::transaction::TransactionSpec.effects` | field | `ferrum/crates/ferrum-state/src/transaction.rs:73` | C5 |
| state.fe.274 | `ferrum_state::transaction::TransactionSpec.annotations` | field | `ferrum/crates/ferrum-state/src/transaction.rs:75` | C5 |
| state.fe.275 | `ferrum_state::transaction::TransactionSpec::new` | method | `ferrum/crates/ferrum-state/src/transaction.rs:80` | C5 |
| state.fe.276 | `ferrum_state::transaction::TransactionSpec::with_changes` | method | `ferrum/crates/ferrum-state/src/transaction.rs:85` | C5 |
| state.fe.277 | `ferrum_state::transaction::TransactionSpec::with_change` | method | `ferrum/crates/ferrum-state/src/transaction.rs:91` | C5 |
| state.fe.278 | `ferrum_state::transaction::TransactionSpec::with_selection` | method | `ferrum/crates/ferrum-state/src/transaction.rs:97` | C5 |
| state.fe.279 | `ferrum_state::transaction::TransactionSpec::with_effect` | method | `ferrum/crates/ferrum-state/src/transaction.rs:103` | C5 |
| state.fe.280 | `ferrum_state::transaction::TransactionSpec::with_annotation` | method | `ferrum/crates/ferrum-state/src/transaction.rs:109` | C5 |
| state.fe.281 | `ferrum_state::transaction::Annotation` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:117` | C5 |
| state.fe.282 | `ferrum_state::transaction::Annotation::of` | method | `ferrum/crates/ferrum-state/src/transaction.rs:124` | C5 |
| state.fe.283 | `ferrum_state::transaction::Annotation::downcast_ref` | method | `ferrum/crates/ferrum-state/src/transaction.rs:132` | C5 |
| state.fe.284 | `ferrum_state::transaction::IsolateHistory` | enum | `ferrum/crates/ferrum-state/src/transaction.rs:143` | C5 |
| state.fe.285 | `ferrum_state::transaction::IsolateHistory::Before` | variant | `ferrum/crates/ferrum-state/src/transaction.rs:144` | C5 |
| state.fe.286 | `ferrum_state::transaction::IsolateHistory::After` | variant | `ferrum/crates/ferrum-state/src/transaction.rs:145` | C5 |
| state.fe.287 | `ferrum_state::transaction::IsolateHistory::Full` | variant | `ferrum/crates/ferrum-state/src/transaction.rs:146` | C5 |
| state.fe.288 | `ferrum_state::transaction::AddToHistory` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:151` | C5 |
| state.fe.289 | `ferrum_state::transaction::AddToHistory.0` | field | `ferrum/crates/ferrum-state/src/transaction.rs:151` | C5 |
| state.fe.290 | `ferrum_state::transaction::UserEvent` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:155` | C5 |
| state.fe.291 | `ferrum_state::transaction::UserEvent.0` | field | `ferrum/crates/ferrum-state/src/transaction.rs:155` | C5 |
| state.fe.292 | `ferrum_state::transaction::TransactionTime` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:159` | C5 |
| state.fe.293 | `ferrum_state::transaction::TransactionTime.0` | field | `ferrum/crates/ferrum-state/src/transaction.rs:159` | C5 |
| state.fe.294 | `ferrum_state::transaction::Transaction` | struct | `ferrum/crates/ferrum-state/src/transaction.rs:164` | C5 |
| state.fe.295 | `ferrum_state::transaction::Transaction::state` | method | `ferrum/crates/ferrum-state/src/transaction.rs:176` | C5 |
| state.fe.296 | `ferrum_state::transaction::Transaction::start_state` | method | `ferrum/crates/ferrum-state/src/transaction.rs:181` | C5 |
| state.fe.297 | `ferrum_state::transaction::Transaction::changes` | method | `ferrum/crates/ferrum-state/src/transaction.rs:186` | C5 |
| state.fe.298 | `ferrum_state::transaction::Transaction::selection` | method | `ferrum/crates/ferrum-state/src/transaction.rs:191` | C5 |
| state.fe.299 | `ferrum_state::transaction::Transaction::effects` | method | `ferrum/crates/ferrum-state/src/transaction.rs:196` | C5 |
| state.fe.300 | `ferrum_state::transaction::Transaction::annotation` | method | `ferrum/crates/ferrum-state/src/transaction.rs:201` | C5 |
| state.fe.301 | `ferrum_state::transaction::Transaction::doc` | method | `ferrum/crates/ferrum-state/src/transaction.rs:211` | C5 |
| state.fe.302 | `ferrum_state::transaction::Transaction::doc_changed` | method | `ferrum/crates/ferrum-state/src/transaction.rs:216` | C5 |
| state.fe.303 | `ferrum_state::transaction::TransactionFilterFn` | type_alias | `ferrum/crates/ferrum-state/src/transaction.rs:222` | C5 |
| state.fe.304 | `ferrum_state::transaction::TRANSACTION_FILTER` | static | `ferrum/crates/ferrum-state/src/transaction.rs:225` | C5 |
| state.fe.305 | `ferrum_state::ChangeDesc` | re-export of change::ChangeDesc | `ferrum/crates/ferrum-state/src/lib.rs:16` | main |
| state.fe.306 | `ferrum_state::ChangeSet` | re-export of change::ChangeSet | `ferrum/crates/ferrum-state/src/lib.rs:16` | main |
| state.fe.307 | `ferrum_state::ChangeSpec` | re-export of change::ChangeSpec | `ferrum/crates/ferrum-state/src/lib.rs:16` | main |
| state.fe.308 | `ferrum_state::Changes` | re-export of change::Changes | `ferrum/crates/ferrum-state/src/lib.rs:16` | main |
| state.fe.309 | `ferrum_state::MapMode` | re-export of change::MapMode | `ferrum/crates/ferrum-state/src/lib.rs:16` | main |
| state.fe.310 | `ferrum_state::count_column` | re-export of column::count_column | `ferrum/crates/ferrum-state/src/lib.rs:17` | main |
| state.fe.311 | `ferrum_state::find_column` | re-export of column::find_column | `ferrum/crates/ferrum-state/src/lib.rs:17` | main |
| state.fe.312 | `ferrum_state::categorize_char` | re-export of char::categorize_char | `ferrum/crates/ferrum-state/src/lib.rs:19` | main |
| state.fe.313 | `ferrum_state::find_cluster_break` | re-export of char::find_cluster_break | `ferrum/crates/ferrum-state/src/lib.rs:19` | main |
| state.fe.314 | `ferrum_state::find_subword_break` | re-export of char::find_subword_break | `ferrum/crates/ferrum-state/src/lib.rs:19` | main |
| state.fe.315 | `ferrum_state::find_word_break` | re-export of char::find_word_break | `ferrum/crates/ferrum-state/src/lib.rs:19` | main |
| state.fe.316 | `ferrum_state::CharCategory` | re-export of char::CharCategory | `ferrum/crates/ferrum-state/src/lib.rs:19` | main |
| state.fe.317 | `ferrum_state::Configuration` | re-export of config::Configuration | `ferrum/crates/ferrum-state/src/lib.rs:21` | main |
| state.fe.318 | `ferrum_state::AnyFacet` | re-export of facet::AnyFacet | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.319 | `ferrum_state::AnyStateField` | re-export of facet::AnyStateField | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.320 | `ferrum_state::Compartment` | re-export of facet::Compartment | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.321 | `ferrum_state::CompartmentId` | re-export of facet::CompartmentId | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.322 | `ferrum_state::Extension` | re-export of facet::Extension | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.323 | `ferrum_state::Facet` | re-export of facet::Facet | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.324 | `ferrum_state::FacetId` | re-export of facet::FacetId | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.325 | `ferrum_state::Prec` | re-export of facet::Prec | `ferrum/crates/ferrum-state/src/lib.rs:23` | main |
| state.fe.326 | `ferrum_state::StateField` | re-export of facet::StateField | `ferrum/crates/ferrum-state/src/lib.rs:24` | main |
| state.fe.327 | `ferrum_state::StateFieldId` | re-export of facet::StateFieldId | `ferrum/crates/ferrum-state/src/lib.rs:24` | main |
| state.fe.328 | `ferrum_state::Chunk` | re-export of rangeset::Chunk | `ferrum/crates/ferrum-state/src/lib.rs:26` | main |
| state.fe.329 | `ferrum_state::Range` | re-export of rangeset::Range | `ferrum/crates/ferrum-state/src/lib.rs:26` | main |
| state.fe.330 | `ferrum_state::RangeSet` | re-export of rangeset::RangeSet | `ferrum/crates/ferrum-state/src/lib.rs:26` | main |
| state.fe.331 | `ferrum_state::RangeSetBuilder` | re-export of rangeset::RangeSetBuilder | `ferrum/crates/ferrum-state/src/lib.rs:26` | main |
| state.fe.332 | `ferrum_state::RangeSetUpdate` | re-export of rangeset::RangeSetUpdate | `ferrum/crates/ferrum-state/src/lib.rs:26` | main |
| state.fe.333 | `ferrum_state::RangeValue` | re-export of rangeset::RangeValue | `ferrum/crates/ferrum-state/src/lib.rs:26` | main |
| state.fe.334 | `ferrum_state::EditorSelection` | re-export of selection::EditorSelection | `ferrum/crates/ferrum-state/src/lib.rs:27` | main |
| state.fe.335 | `ferrum_state::SelectionRange` | re-export of selection::SelectionRange | `ferrum/crates/ferrum-state/src/lib.rs:27` | main |
| state.fe.336 | `ferrum_state::ChangeByRangeResult` | re-export of state::ChangeByRangeResult | `ferrum/crates/ferrum-state/src/lib.rs:28` | main |
| state.fe.337 | `ferrum_state::EditorState` | re-export of state::EditorState | `ferrum/crates/ferrum-state/src/lib.rs:28` | main |
| state.fe.338 | `ferrum_state::EditorStateConfig` | re-export of state::EditorStateConfig | `ferrum/crates/ferrum-state/src/lib.rs:28` | main |
| state.fe.339 | `ferrum_state::RangeResult` | re-export of state::RangeResult | `ferrum/crates/ferrum-state/src/lib.rs:28` | main |
| state.fe.340 | `ferrum_state::Line` | re-export of text::Line | `ferrum/crates/ferrum-state/src/lib.rs:29` | main |
| state.fe.341 | `ferrum_state::Text` | re-export of text::Text | `ferrum/crates/ferrum-state/src/lib.rs:29` | main |
| state.fe.342 | `ferrum_state::AddToHistory` | re-export of transaction::AddToHistory | `ferrum/crates/ferrum-state/src/lib.rs:31` | main |
| state.fe.343 | `ferrum_state::Annotation` | re-export of transaction::Annotation | `ferrum/crates/ferrum-state/src/lib.rs:31` | main |
| state.fe.344 | `ferrum_state::CompartmentReconfigure` | re-export of transaction::CompartmentReconfigure | `ferrum/crates/ferrum-state/src/lib.rs:31` | main |
| state.fe.345 | `ferrum_state::IsolateHistory` | re-export of transaction::IsolateHistory | `ferrum/crates/ferrum-state/src/lib.rs:31` | main |
| state.fe.346 | `ferrum_state::StateEffect` | re-export of transaction::StateEffect | `ferrum/crates/ferrum-state/src/lib.rs:31` | main |
| state.fe.347 | `ferrum_state::Transaction` | re-export of transaction::Transaction | `ferrum/crates/ferrum-state/src/lib.rs:31` | main |
| state.fe.348 | `ferrum_state::TransactionSpec` | re-export of transaction::TransactionSpec | `ferrum/crates/ferrum-state/src/lib.rs:32` | main |
| state.fe.349 | `ferrum_state::TransactionTime` | re-export of transaction::TransactionTime | `ferrum/crates/ferrum-state/src/lib.rs:32` | main |
| state.fe.350 | `ferrum_state::UserEvent` | re-export of transaction::UserEvent | `ferrum/crates/ferrum-state/src/lib.rs:32` | main |
| state.fe.351 | `ferrum_state::TRANSACTION_FILTER` | re-export of transaction::TRANSACTION_FILTER | `ferrum/crates/ferrum-state/src/lib.rs:32` | main |
| state.fe.352 | `ferrum_state::TransactionFilterFn` | re-export of transaction::TransactionFilterFn | `ferrum/crates/ferrum-state/src/lib.rs:32` | main |
| state.fe.353 | `ferrum_state::text::Text as Default` | std trait impl | `ferrum/crates/ferrum-state/src/text.rs:13` | C1 |
| state.fe.354 | `ferrum_state::text::Text as Display` | std trait impl | `ferrum/crates/ferrum-state/src/text.rs:200` | C1 |
| state.fe.355 | `ferrum_state::text::Text as Debug` | std trait impl | `ferrum/crates/ferrum-state/src/text.rs:206` | C1 |
| state.fe.356 | `ferrum_state::text::Text as From<&str>` | std trait impl | `ferrum/crates/ferrum-state/src/text.rs:218` | C1 |
| state.fe.357 | `ferrum_state::text::Text as From<String>` | std trait impl | `ferrum/crates/ferrum-state/src/text.rs:224` | C1 |
| state.fe.358 | `ferrum_state::change::ChangeDesc as Display` | std trait impl | `ferrum/crates/ferrum-state/src/change.rs:216` | C2 |
| state.fe.359 | `ferrum_state::change::ChangeDesc as Debug` | std trait impl | `ferrum/crates/ferrum-state/src/change.rs:238` | C2 |
| state.fe.360 | `ferrum_state::change::Changes as From<ChangeSet>` | std trait impl | `ferrum/crates/ferrum-state/src/change.rs:300` | C2 |
| state.fe.361 | `ferrum_state::change::Changes as From<ChangeSpec>` | std trait impl | `ferrum/crates/ferrum-state/src/change.rs:306` | C2 |
| state.fe.362 | `ferrum_state::change::Changes as From<Vec<ChangeSpec>>` | std trait impl | `ferrum/crates/ferrum-state/src/change.rs:312` | C2 |
| state.fe.363 | `ferrum_state::change::ChangeSet as Display` | std trait impl | `ferrum/crates/ferrum-state/src/change.rs:541` | C2 |
| state.fe.364 | `ferrum_state::change::ChangeSet as Debug` | std trait impl | `ferrum/crates/ferrum-state/src/change.rs:547` | C2 |
| state.fe.365 | `ferrum_state::selection::EditorSelection as Default` | std trait impl | `ferrum/crates/ferrum-state/src/selection.rs:122` | C3 |
| state.fe.366 | `ferrum_state::facet::Compartment as Default` | std trait impl | `ferrum/crates/ferrum-state/src/facet.rs:222` | C4 |
| state.fe.367 | `ferrum_state::facet::Extension as From<Vec<Extension>>` | std trait impl | `ferrum/crates/ferrum-state/src/facet.rs:274` | C4 |
| state.fe.368 | `ferrum_state::rangeset::RangeSet as Default` | std trait impl | `ferrum/crates/ferrum-state/src/rangeset.rs:99` | C6 |
| state.fe.369 | `ferrum_state::rangeset::RangeSetUpdate as Default` | std trait impl | `ferrum/crates/ferrum-state/src/rangeset.rs:282` | C6 |
| state.fe.370 | `ferrum_state::rangeset::RangeSetBuilder as Default` | std trait impl | `ferrum/crates/ferrum-state/src/rangeset.rs:311` | C6 |
| state.fe.371 | `ferrum_state::transaction::StateEffect as Debug` | std trait impl | `ferrum/crates/ferrum-state/src/transaction.rs:16` | C5 |
| state.fe.372 | `ferrum::state` | facade re-export of crate ferrum_state | `ferrum/crates/ferrum/src/lib.rs:3` | main |
| state.fe.373 | `ferrum::ChangeSet` | facade re-export of ferrum_state::ChangeSet | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.374 | `ferrum::ChangeSpec` | facade re-export of ferrum_state::ChangeSpec | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.375 | `ferrum::Compartment` | facade re-export of ferrum_state::Compartment | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.376 | `ferrum::EditorSelection` | facade re-export of ferrum_state::EditorSelection | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.377 | `ferrum::EditorState` | facade re-export of ferrum_state::EditorState | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.378 | `ferrum::EditorStateConfig` | facade re-export of ferrum_state::EditorStateConfig | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.379 | `ferrum::Extension` | facade re-export of ferrum_state::Extension | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.380 | `ferrum::Facet` | facade re-export of ferrum_state::Facet | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.381 | `ferrum::Prec` | facade re-export of ferrum_state::Prec | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.382 | `ferrum::Range` | facade re-export of ferrum_state::Range | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.383 | `ferrum::RangeSet` | facade re-export of ferrum_state::RangeSet | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.384 | `ferrum::RangeSetBuilder` | facade re-export of ferrum_state::RangeSetBuilder | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.385 | `ferrum::RangeValue` | facade re-export of ferrum_state::RangeValue | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.386 | `ferrum::SelectionRange` | facade re-export of ferrum_state::SelectionRange | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.387 | `ferrum::StateEffect` | facade re-export of ferrum_state::StateEffect | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.388 | `ferrum::StateField` | facade re-export of ferrum_state::StateField | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.389 | `ferrum::Text` | facade re-export of ferrum_state::Text | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.390 | `ferrum::Transaction` | facade re-export of ferrum_state::Transaction | `ferrum/crates/ferrum/src/lib.rs:12` | main |
| state.fe.391 | `ferrum::TransactionSpec` | facade re-export of ferrum_state::TransactionSpec | `ferrum/crates/ferrum/src/lib.rs:12` | main |
