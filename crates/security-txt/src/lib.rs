#[cfg(feature = "parse")]
pub mod parse;

/// Includes a security.txt inside your program binary.
///
/// # Example
///
/// ```
/// #[cfg(not(feature = "no-entrypoint"))]
/// security_txt::security_txt! {
///     // Required fields.
///     name: "My Program",
///     project_url: "https://example.com",
///     contacts: "email:me@example.com",
///     policy: "https://example.com/security",
///
///     // Optional fields.
///     preferred_languages: "",
///     encryption: "",
///     source_code: "",
///     source_release: "",
///     source_revision: "",
///     auditors: "",
///     acknowledgements: "",
///     expiry: "",
/// }
/// ```
#[macro_export]
macro_rules! security_txt {
    ($($field:ident: $value:expr),* $(,)?) => {
        #[allow(unexpected_cfgs)]
        const _: () = {
            // Check fields are valid and without duplicates.
            let _ = || {
                #[derive(Default)]
                struct SecurityTxt {
                    /// The name of the project.
                    ///
                    /// If the project isn't public, you can put `private`.
                    name: &'static str,
                    /// A URL to the project's homepage/dapp.
                    ///
                    /// If the project isn't public, you can put `private`.
                    project_url: &'static str,
                    /// A comma-separated list of contact information in the format
                    /// `<type>:<information>`.
                    ///
                    /// Possible contact types are: `email`, `link`, `discord`, `telegram`,
                    /// `twitter`, and `other`.
                    ///
                    /// The list should be roughly in order of preference. Prefer contact types that
                    /// likely won't change for a while, like a `security@example.com` email address.
                    contacts: &'static str,
                    /// A link to or text document describing the project's security policy.
                    policy: &'static str,
                    /// A comma-separated list of preferred languages (ISO 639-1).
                    preferred_languages: &'static str,
                    /// A PGP public key block (or similar) or a link to one.
                    encryption: &'static str,
                    /// A URL to the project's source code.
                    source_code: &'static str,
                    /// The release identifier for this build.
                    ///
                    /// Ideally corresponding to a Git tag that can be rebuilt to reproduce the same
                    /// binary. 3rd party build verification tools may will this tag to identify a
                    /// matching GitHub release.
                    source_release: &'static str,
                    /// The revision identifier for this build.
                    ///
                    /// Ideally corresponding to a Git SHA that can be rebuilt to reproduce the same
                    /// binary. 3rd party build verification tools may will this tag to identify a
                    /// matching GitHub release.
                    source_revision: &'static str,
                    /// A comma-separated list of people or entities that audited this program, or
                    /// a link to a page where audit reports are hosted.
                    auditors: &'static str,
                    /// A link to or text document containing acknowledgements to security
                    /// researchers who have previously found vulnerabilities in the project.
                    acknowledgements: &'static str,
                    /// The date the security.txt will expire, in `YYYY-MM-DD` format.
                    expiry: &'static str,
                }

                #[allow(clippy::needless_update)]
                let _ = SecurityTxt {
                    $($field: $value,)*
                    ..Default::default()
                };

            };

            // Check all required fields are present.
            $crate::__private_required_fields! {
                input = [{ $($field)* }]
            }

            #[allow(dead_code)]
            #[no_mangle]
            #[cfg_attr(target_os = "solana", link_section = ".security.txt")]
            static security_txt: &str = concat! {
                "=======BEGIN SECURITY.TXT V1=======\0",
                $(stringify!($field), "\0", $value, "\0",)*
                "=======END SECURITY.TXT V1=======\0"
            };
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __private_required_fields {
    // End of input. Check required fields.
    (
        required = [{ $($required:ident)* }]
        input = [{ }]
    ) => {
        const _: () = {
            struct Required {
                name: (),
                project_url: (),
                contacts: (),
                policy: (),
            }

            let _ = Required {
                $($required: (),)*
            };
        };
    };

    // Entrypoint.
    (
        input = [{ $($input:ident)* }]
    ) => {
        $crate::__private_required_fields! {
            required = [{  }]
            input = [{ $($input)* }]
        }
    };

    (
        required = [{ $($required:ident)* }]
        input = [{ name $($input:ident)* }]
    ) => {
        $crate::__private_required_fields! {
            required = [{ $($required)* name }]
            input = [{ $($input)* }]
        }
    };
    (
        required = [{ $($required:ident)* }]
        input = [{ project_url $($input:ident)* }]
    ) => {
        $crate::__private_required_fields! {
            required = [{ $($required)* project_url }]
            input = [{ $($input)* }]
        }
    };
    (
        required = [{ $($required:ident)* }]
        input = [{ contacts $($input:ident)* }]
    ) => {
        $crate::__private_required_fields! {
            required = [{ $($required)* contacts }]
            input = [{ $($input)* }]
        }
    };
    (
        required = [{ $($required:ident)* }]
        input = [{ policy $($input:ident)* }]
    ) => {
        $crate::__private_required_fields! {
            required = [{ $($required)* policy }]
            input = [{ $($input)* }]
        }
    };
    (
        required = [{ $($required:ident)* }]
        input = [{ $other:ident $($input:ident)* }]
    ) => {
        $crate::__private_required_fields! {
            required = [{ $($required)* }]
            input = [{ $($input)* }]
        }
    };
}
