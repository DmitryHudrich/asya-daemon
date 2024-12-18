# macros

The macros rust crate that contain (currenly) single derive macro - `Property`.
Usually this crate uses as helper for writing a bunch of boilerplate code (e.g.
parsing config values).

Below you'll see the description of macros from this crate.

## Property derive macro

The macro which uses clean config struct and creates new struct with field types
wrapped by `Option<T>`.

Let we pick a struct and attach the macro:

```rust
#[derive(Property)]
#[property(name(ConfigProperty))]
struct Config {
    enable_smth: bool,
    parameter: String,
}
```

It will create a new struct in background:

```rust
struct ConfigProperty {
    enable_smth: Option<bool>,
    parameter: Option<String>,
}
```

The second struct will be helpful if you want to parse the values using `serde`
crate. So if you need to attach the derive macros to new struct, need to use
`derive` argument of `property` attribute. The changes:

```rust
#[property(name(ConfigProperty), derive(Deserialize, Clone, Default))]
struct Config {
```

As you can see, there is also `Clone` and `Default` derive traits. It is intentional,
because the new method of this trait requires that the struct can be cloned. And
the `Default` trait uses for unwrapping values (it will be described below).

But it won't compile and work because we didn't mark fields that can be default:

```rust
struct Config {
    #[property(default(true))]
    enable_smth: bool,

    #[property(default(path = String::new))]
    parameter: String,
}
```

Now the code should be able to compile.

> [!NOTE]
> Also you can omit the parenthesis and just write `#[property(default)]`, it will use
> `Default` trait to do this.

Alright, except this, the macro also provide the methods for new struct which must
help reduce a bunch of boilerplate code. First and main method - `unwrap_or_default(self) -> OriginalStruct`.
This method unwraps the fields and sets default values if these is `None`.

It will look like that in generated code, considering to used attributes:

```rust
impl ConfigProperty {
    fn unwrap_or_default(self) -> Config {
        let Self { enable_smth, parameter } = self;
        Config {
            enable_smth: enable_smth.unwrap_or(true),
            parameter: parameter.unwrap_or_else(String::new),
        }
    }
}
```

With it also implemented `From<ConfigProperty> for Config` trait:

```rust
impl From<ConfigProperty> for Config {
    fn from(value: ConfigProperty) -> Self {
        value.unwrap_or_default()
    }
}
```

You can use `unwrap_or_default(self)` and `into()` together. It was made for ease about which
will be described below.

Also the created struct should be able to merge with other using `merge(self, other: Option<Self>) -> Self`
method. Note that the method consumes current value and creates new struct from both values.
Usually it more easier than use mutable reference, especially if there is _submerges_.

Merging struct looks like:

```rust
impl ConfigProperty {
    fn merge(self, other: Option<Self>) -> Self {
        let Some(other) = other else {
            return self;
        }

        let Self { enable_smth, parameter } = self;
        Self {
            enable_smth: enable_smth.or(other.enable_smth),
            parameter: parameter.or(other.parameter),
        }
    }
}
```

The examples were simple and understandable. But when you want to have a struct that have
inner struct, like composion. In this examples, we'll add the new struct:

```rust
#[derive(Property)]
#[property(name(AudioConfigProperty), derive(Deserialize, Clone, Default))]
struct AudioConfig {
    enabled: bool,
    volume: u8,
}
```

But there is a problem. If we use `AudioConfig` in `Config` struct, the code will not
compile because the `AudioConfig` doesn't have `Deserialize` derive trait. But using
`AudioConfigProperty` in `Config` struct is not a such good idea. And there is a solution:

```rust
struct Config {
    // other fields omitted
    #[property(use_type(AudioConfigProperty), mergeable)]
    audio: AudioConfig,
}
```

As you see, the `use_type` argument tells the macro that need to use `AudioConfigProperty`
type instead of `AudioConfig` in `ConfigProperty` struct. The main contract that for
the used type should be implemented a `From<UsedType> for Type` trait. As I've written above,
the `Property` macro autogenerates this trait, so you should'nt mind about it.

There is also detail about attribute - `mergeable`. It tells the macro that there is a
type that have `merge` method and use it if possible. For instance, you have the different
values for `AudioConfig` from different sources but you want to merge them, then `mergeable`
is that what you need. And it merges `AudioConfigProperty` from first struct with `AudioConfigProperty`
from second struct using `merge` method of `ConfigProperty` struct.

In last, there is also a way to verify values after all possible merging. In this context,
the verifier is not powerful method but extensible. And it doesn't check the `None`s in
sturct. This verifier was made for other purpose - verify values by complex algorithm and
return error if there is rude mistakes or do nothing if there is no issues.

To verify, use the `verify(&self)` method. Initially this method is empty, because we didn't
mark any fields by argument `verifier(composite)` or `verifier(path = path::to::function)`.
For example, you can verify two fields:

```rust
struct Config {
    // all other fields are omitted
    #[property(default(path = String::new), verifier(path = crate::module::check_parameter))]
    parameter: String,

    #[property(
        use_type(AudioConfigProperty),
        mergeable,
        verifier(composite)
    )]
    audio: AudioConfig,
}

struct AudioConfig {
    #[property(verifier(path = crate::audio::check_volume))]
    volume: u8,
}
```

There is three different places of `verifier` argument. For `parameter` field it will call
`crate::module::check_parameter` function with passing `Option<&String>` value. The
function should have return type like `Result<(), Box<dyn std::error::Error>` for easing
forwarding to main `verify(&self)` function.

> [!NOTE]
> The functions which used to verify fields should have such `fn(Option<&T>) -> Result<(), Box<dyn std::error::Error>`
> signature where `T` is type of verifying field.

Also there is `crate::audio::check_volume` which should be called in `verify` method for
`AudioConfig`. But there is a tricky part - the macro doesn't know about the `verify` method
of `AudioConfig` and will not call it. So to tell it need to use `verifier(composite)` that
tells the macro to use `verify` method of concrete type (in this case - `AudioConfig`).

> [!NOTE]
> The `verifier(composite)` is soft during verifying because if the field value is `None`
> it will be skipped. So if you want to throw error if there is `None` in field value, need
> to use `verifier(composite, force_check)` with which will return error if there is `None`.

### Conclusions

Generated methods:

- `unwrap_or_default(self) -> OriginalStruct` - unwraps all fields or uses default
  values instead and converts into original struct.
- `merge(self, other: Option<Self>) -> Self` - merges the current struct with other.
- `verify(&self) -> Result<(), Box<dyn std::error::Error>` - verifies the struct.

Generated impls:

- `impl From<Property> to Origin`

Possible attributes:

- `default`, `default(expression)` or `default(path = path::to::function)` - marker that
  tells that the field can have default value using `Default` trait, expression or function respectively.
- `use_type(TypeName)` - uses other type that can `Into` to field.
- `mergeable` - marker that tells that the field can use `merge` in `merge` function of struct.
- `verifier(composite)`, `verifier(composite, force_check)` or `verifier(path = path::to::function)` - marker
  that tells that the field should be verified by provided function or use inner verifier if it is composite.
