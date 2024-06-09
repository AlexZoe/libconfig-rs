# libconfig-rs

Rust wrapper of	the C++ [libconfig](https://github.com/hyperrealm/libconfig) library.

## Implementation Progress

### Config Methods

- [ ] read
- [ ] write
- [x] readFile
- [ ] writeFile
- [ ] readString
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
- [ ] add
- [ ] remove
- [ ] remove(index)
- [x] getName
- [ ] getPath
- [ ] getParent
- [ ] isRoot
- [ ] getIndex
- [ ] getType
- [ ] getFormat
- [x] exists
- [ ] iterator
- [ ] getLength
- [ ] isGroup
- [ ] isArray
- [ ] isList
- [ ] isAggregrate
- [ ] isScalar
- [ ] isNumber
- [ ] isString
- [ ] getSourceFile
- [ ] getSourceLine
