# libconfig-rs

Rust wrapper of	the C++ [libconfig](https://github.com/hyperrealm/libconfig) library.

## Implementation Progress

### Config Methods

- [ ] read
- [ ] write
- [x] readFile
- [x] writeFile
- [x] readString
- [x] setIncludeDir
- [ ] getIncludeDir
- [ ] getOptions
- [ ] setOptions
- [ ] getOption
- [ ] setOption
- [ ] getDefaultFormat
- [ ] setDefaultFormat
- [ ] setTabWidth
- [ ] getTabWidth
- [ ] getFloatPrecision
- [ ] setFloatPrecision
- [x] getRoot
- [x] lookup
- [x] exists
- [x] lookupValue (bool)
- [x] lookupValue (int)
- [x] lookupValue (long long)
- [x] lookupValue (float)
- [x] lookupValue (double)
- [x] lookupValue (std::string)

### Setting Methods

- [x] lookup
- [x] lookupValue (bool)
- [x] lookupValue (int)
- [x] lookupValue (long long)
- [x] lookupValue (float)
- [x] lookupValue (double)
- [x] lookupValue (std::string)
- [x] add
- [x] remove
- [x] remove(index)
- [x] getName
- [x] getPath
- [x] getParent
- [x] isRoot
- [x] getIndex
- [x] getType
- [ ] getFormat
- [x] exists
- [x] iterator
- [x] getLength
- [x] isGroup
- [x] isArray
- [x] isList
- [x] isAggregate
- [x] isScalar
- [x] isNumber
- [x] isString
- [ ] getSourceFile
- [ ] getSourceLine
