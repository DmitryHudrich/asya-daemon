#[macro_export]
macro_rules! paraminfo {
    (
        $(
            $primary:tt: [
                $(
                    $secondary:tt: [
                        $(
                            $tertiary:tt: [
                                $(
                                    $property:expr, $handler:expr
                                );* $(;)?
                            ]
                        );* $(;)?
                    ]
                );* $(;)?
            ]
        );* $(;)?
    ) => {{
        let mut params = Vec::new();
        $(
            $(
                $(
                    $(
                        params.push(ParamInfo {
                            primary_type: paraminfo_stringify!($primary),
                            secondary_type: paraminfo_stringify!($secondary),
                            tertiary_type: paraminfo_stringify!($tertiary),
                            property: $property.to_string(),
                            handler: $handler,
                        });
                    )*
                )*
            )*
        )*
        params
    }};
}

#[macro_export]
macro_rules! paraminfo_stringify {
    ($x:tt) => {{
        #[allow(unused_braces)]
        match $x {
            x if x == stringify!($x) => x.to_string(),
            _ => $x.to_string(),
        }
    }};
}
