#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Similar to the standard date type, but each section after the year is optional. e.g. 2014-06-29 or 2023-04"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Similar to the standard date type, but each section after the year is optional. e.g. 2014-06-29 or 2023-04\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Iso8601(::std::string::String);
impl ::std::ops::Deref for Iso8601 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Iso8601> for ::std::string::String {
    fn from(value: Iso8601) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Iso8601> for Iso8601 {
    fn from(value: &Iso8601) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Iso8601 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new(
            "^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$",
        )
        .unwrap()
        .find(value)
        .is_none()
        {
            return Err ("doesn't match pattern \"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Iso8601 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Iso8601 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Iso8601 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Iso8601 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "ResumeSchema"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"http://example.com/example.json\","]
#[doc = "  \"title\": \"Resume Schema\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"description\": \"link to the version of the schema that can validate the resume\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"awards\": {"]
#[doc = "      \"description\": \"Specify any awards you have received throughout your professional career\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"awarder\": {"]
#[doc = "            \"description\": \"e.g. Time Magazine\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"date\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"summary\": {"]
#[doc = "            \"description\": \"e.g. Received for my work with Quantum Physics\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"title\": {"]
#[doc = "            \"description\": \"e.g. One of the 100 greatest minds of the century\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"basics\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"email\": {"]
#[doc = "          \"description\": \"e.g. thomas@gmail.com\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"email\""]
#[doc = "        },"]
#[doc = "        \"image\": {"]
#[doc = "          \"description\": \"URL (as per RFC 3986) to a image in JPEG or PNG format\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"label\": {"]
#[doc = "          \"description\": \"e.g. Web Developer\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"location\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"address\": {"]
#[doc = "              \"description\": \"To add multiple address lines, use \\n. For example, 1234 Glücklichkeit Straße\\nHinterhaus 5. Etage li.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"city\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"countryCode\": {"]
#[doc = "              \"description\": \"code as per ISO-3166-1 ALPHA-2, e.g. US, AU, IN\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"postalCode\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"region\": {"]
#[doc = "              \"description\": \"The general region where you live. Can be a US state, or a province, for instance.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": true"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"phone\": {"]
#[doc = "          \"description\": \"Phone numbers are stored as strings so use any format you like, e.g. 712-117-2923\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"profiles\": {"]
#[doc = "          \"description\": \"Specify any number of social networks that you participate in\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"network\": {"]
#[doc = "                \"description\": \"e.g. Facebook or Twitter\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"url\": {"]
#[doc = "                \"description\": \"e.g. http://twitter.example.com/neutralthoughts\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"format\": \"uri\""]
#[doc = "              },"]
#[doc = "              \"username\": {"]
#[doc = "                \"description\": \"e.g. neutralthoughts\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": true"]
#[doc = "          },"]
#[doc = "          \"additionalItems\": false"]
#[doc = "        },"]
#[doc = "        \"summary\": {"]
#[doc = "          \"description\": \"Write a short 2-3 sentence biography about yourself\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"description\": \"URL (as per RFC 3986) to your website, e.g. personal homepage\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"certificates\": {"]
#[doc = "      \"description\": \"Specify any certificates you have received throughout your professional career\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"date\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"issuer\": {"]
#[doc = "            \"description\": \"e.g. CNCF\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"e.g. Certified Kubernetes Administrator\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"e.g. http://example.com\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"education\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"area\": {"]
#[doc = "            \"description\": \"e.g. Arts\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"courses\": {"]
#[doc = "            \"description\": \"List notable courses/subjects\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. H1302 - Introduction to American history\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"endDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"institution\": {"]
#[doc = "            \"description\": \"e.g. Massachusetts Institute of Technology\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"score\": {"]
#[doc = "            \"description\": \"grade point average, e.g. 3.67/4.0\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"startDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"studyType\": {"]
#[doc = "            \"description\": \"e.g. Bachelor\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"e.g. http://facebook.example.com\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"interests\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"keywords\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. Friedrich Nietzsche\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"e.g. Philosophy\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"languages\": {"]
#[doc = "      \"description\": \"List any other languages you speak\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"fluency\": {"]
#[doc = "            \"description\": \"e.g. Fluent, Beginner\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"language\": {"]
#[doc = "            \"description\": \"e.g. English, Spanish\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"meta\": {"]
#[doc = "      \"description\": \"The schema version and any other tooling configuration lives here\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"canonical\": {"]
#[doc = "          \"description\": \"URL (as per RFC 3986) to latest version of this document\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        },"]
#[doc = "        \"lastModified\": {"]
#[doc = "          \"description\": \"Using ISO 8601 with YYYY-MM-DDThh:mm:ss\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"description\": \"A version field which follows semver - e.g. v1.0.0\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"projects\": {"]
#[doc = "      \"description\": \"Specify career projects\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"description\": {"]
#[doc = "            \"description\": \"Short summary of project. e.g. Collated works of 2017.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"endDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"entity\": {"]
#[doc = "            \"description\": \"Specify the relevant company/entity affiliations e.g. 'greenpeace', 'corporationXYZ'\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"highlights\": {"]
#[doc = "            \"description\": \"Specify multiple features\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. Directs you close but not quite there\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"keywords\": {"]
#[doc = "            \"description\": \"Specify special elements involved\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. AngularJS\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"e.g. The World Wide Web\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"roles\": {"]
#[doc = "            \"description\": \"Specify your role on this project or in company\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. Team Lead, Speaker, Writer\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"startDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"description\": \" e.g. 'volunteering', 'presentation', 'talk', 'application', 'conference'\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"e.g. http://www.computer.org/csdl/mags/co/1996/10/rx069-abs.html\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"publications\": {"]
#[doc = "      \"description\": \"Specify your publications through your career\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"e.g. The World Wide Web\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"publisher\": {"]
#[doc = "            \"description\": \"e.g. IEEE, Computer Magazine\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"releaseDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"summary\": {"]
#[doc = "            \"description\": \"Short summary of publication. e.g. Discussion of the World Wide Web, HTTP, HTML.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"e.g. http://www.computer.org.example.com/csdl/mags/co/1996/10/rx069-abs.html\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"references\": {"]
#[doc = "      \"description\": \"List references you have received\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"e.g. Timothy Cook\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"reference\": {"]
#[doc = "            \"description\": \"e.g. Joe blogs was a great employee, who turned up to work at least once a week. He exceeded my expectations when it came to doing nothing.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"skills\": {"]
#[doc = "      \"description\": \"List out your professional skill-set\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"keywords\": {"]
#[doc = "            \"description\": \"List some keywords pertaining to this skill\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. HTML\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"level\": {"]
#[doc = "            \"description\": \"e.g. Master\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"e.g. Web Development\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"volunteer\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"endDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"highlights\": {"]
#[doc = "            \"description\": \"Specify accomplishments and achievements\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. Increased profits by 20% from 2011-2012 through viral advertising\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"organization\": {"]
#[doc = "            \"description\": \"e.g. Facebook\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"position\": {"]
#[doc = "            \"description\": \"e.g. Software Engineer\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"startDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"summary\": {"]
#[doc = "            \"description\": \"Give an overview of your responsibilities at the company\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"e.g. http://facebook.example.com\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"work\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"description\": {"]
#[doc = "            \"description\": \"e.g. Social Media Company\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"endDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"highlights\": {"]
#[doc = "            \"description\": \"Specify multiple accomplishments\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"description\": \"e.g. Increased profits by 20% from 2011-2012 through viral advertising\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"additionalItems\": false"]
#[doc = "          },"]
#[doc = "          \"location\": {"]
#[doc = "            \"description\": \"e.g. Menlo Park, CA\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
#[doc = "            \"description\": \"e.g. Facebook\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"position\": {"]
#[doc = "            \"description\": \"e.g. Software Engineer\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"startDate\": {"]
#[doc = "            \"$ref\": \"#/definitions/iso8601\""]
#[doc = "          },"]
#[doc = "          \"summary\": {"]
#[doc = "            \"description\": \"Give an overview of your responsibilities at the company\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"e.g. http://facebook.example.com\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchema {
    #[doc = "Specify any awards you have received throughout your professional career"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub awards: ::std::vec::Vec<ResumeSchemaAwardsItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub basics: ::std::option::Option<ResumeSchemaBasics>,
    #[doc = "Specify any certificates you have received throughout your professional career"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub certificates: ::std::vec::Vec<ResumeSchemaCertificatesItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub education: ::std::vec::Vec<ResumeSchemaEducationItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub interests: ::std::vec::Vec<ResumeSchemaInterestsItem>,
    #[doc = "List any other languages you speak"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<ResumeSchemaLanguagesItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub meta: ::std::option::Option<ResumeSchemaMeta>,
    #[doc = "Specify career projects"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<ResumeSchemaProjectsItem>,
    #[doc = "Specify your publications through your career"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub publications: ::std::vec::Vec<ResumeSchemaPublicationsItem>,
    #[doc = "List references you have received"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub references: ::std::vec::Vec<ResumeSchemaReferencesItem>,
    #[doc = "link to the version of the schema that can validate the resume"]
    #[serde(
        rename = "$schema",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub schema: ::std::option::Option<::std::string::String>,
    #[doc = "List out your professional skill-set"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<ResumeSchemaSkillsItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub volunteer: ::std::vec::Vec<ResumeSchemaVolunteerItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub work: ::std::vec::Vec<ResumeSchemaWorkItem>,
}
impl ::std::convert::From<&ResumeSchema> for ResumeSchema {
    fn from(value: &ResumeSchema) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchema {
    fn default() -> Self {
        Self {
            awards: Default::default(),
            basics: Default::default(),
            certificates: Default::default(),
            education: Default::default(),
            interests: Default::default(),
            languages: Default::default(),
            meta: Default::default(),
            projects: Default::default(),
            publications: Default::default(),
            references: Default::default(),
            schema: Default::default(),
            skills: Default::default(),
            volunteer: Default::default(),
            work: Default::default(),
        }
    }
}
impl ResumeSchema {
    pub fn builder() -> builder::ResumeSchema {
        Default::default()
    }
}
#[doc = "ResumeSchemaAwardsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"awarder\": {"]
#[doc = "      \"description\": \"e.g. Time Magazine\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"e.g. Received for my work with Quantum Physics\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"e.g. One of the 100 greatest minds of the century\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaAwardsItem {
    #[doc = "e.g. Time Magazine"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub awarder: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<Iso8601>,
    #[doc = "e.g. Received for my work with Quantum Physics"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. One of the 100 greatest minds of the century"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaAwardsItem> for ResumeSchemaAwardsItem {
    fn from(value: &ResumeSchemaAwardsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaAwardsItem {
    fn default() -> Self {
        Self {
            awarder: Default::default(),
            date: Default::default(),
            summary: Default::default(),
            title: Default::default(),
        }
    }
}
impl ResumeSchemaAwardsItem {
    pub fn builder() -> builder::ResumeSchemaAwardsItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaBasics"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"email\": {"]
#[doc = "      \"description\": \"e.g. thomas@gmail.com\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"email\""]
#[doc = "    },"]
#[doc = "    \"image\": {"]
#[doc = "      \"description\": \"URL (as per RFC 3986) to a image in JPEG or PNG format\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"description\": \"e.g. Web Developer\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"address\": {"]
#[doc = "          \"description\": \"To add multiple address lines, use \\n. For example, 1234 Glücklichkeit Straße\\nHinterhaus 5. Etage li.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"city\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"countryCode\": {"]
#[doc = "          \"description\": \"code as per ISO-3166-1 ALPHA-2, e.g. US, AU, IN\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"postalCode\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"region\": {"]
#[doc = "          \"description\": \"The general region where you live. Can be a US state, or a province, for instance.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"phone\": {"]
#[doc = "      \"description\": \"Phone numbers are stored as strings so use any format you like, e.g. 712-117-2923\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"profiles\": {"]
#[doc = "      \"description\": \"Specify any number of social networks that you participate in\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"network\": {"]
#[doc = "            \"description\": \"e.g. Facebook or Twitter\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"e.g. http://twitter.example.com/neutralthoughts\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\""]
#[doc = "          },"]
#[doc = "          \"username\": {"]
#[doc = "            \"description\": \"e.g. neutralthoughts\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"Write a short 2-3 sentence biography about yourself\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"URL (as per RFC 3986) to your website, e.g. personal homepage\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaBasics {
    #[doc = "e.g. thomas@gmail.com"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "URL (as per RFC 3986) to a image in JPEG or PNG format"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Web Developer"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<ResumeSchemaBasicsLocation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Phone numbers are stored as strings so use any format you like, e.g. 712-117-2923"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone: ::std::option::Option<::std::string::String>,
    #[doc = "Specify any number of social networks that you participate in"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub profiles: ::std::vec::Vec<ResumeSchemaBasicsProfilesItem>,
    #[doc = "Write a short 2-3 sentence biography about yourself"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "URL (as per RFC 3986) to your website, e.g. personal homepage"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaBasics> for ResumeSchemaBasics {
    fn from(value: &ResumeSchemaBasics) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaBasics {
    fn default() -> Self {
        Self {
            email: Default::default(),
            image: Default::default(),
            label: Default::default(),
            location: Default::default(),
            name: Default::default(),
            phone: Default::default(),
            profiles: Default::default(),
            summary: Default::default(),
            url: Default::default(),
        }
    }
}
impl ResumeSchemaBasics {
    pub fn builder() -> builder::ResumeSchemaBasics {
        Default::default()
    }
}
#[doc = "ResumeSchemaBasicsLocation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"address\": {"]
#[doc = "      \"description\": \"To add multiple address lines, use \\n. For example, 1234 Glücklichkeit Straße\\nHinterhaus 5. Etage li.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"city\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"countryCode\": {"]
#[doc = "      \"description\": \"code as per ISO-3166-1 ALPHA-2, e.g. US, AU, IN\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"postalCode\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"region\": {"]
#[doc = "      \"description\": \"The general region where you live. Can be a US state, or a province, for instance.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaBasicsLocation {
    #[doc = "To add multiple address lines, use \n. For example, 1234 Glücklichkeit Straße\nHinterhaus 5. Etage li."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: ::std::option::Option<::std::string::String>,
    #[doc = "code as per ISO-3166-1 ALPHA-2, e.g. US, AU, IN"]
    #[serde(
        rename = "countryCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "postalCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[doc = "The general region where you live. Can be a US state, or a province, for instance."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub region: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaBasicsLocation> for ResumeSchemaBasicsLocation {
    fn from(value: &ResumeSchemaBasicsLocation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaBasicsLocation {
    fn default() -> Self {
        Self {
            address: Default::default(),
            city: Default::default(),
            country_code: Default::default(),
            postal_code: Default::default(),
            region: Default::default(),
        }
    }
}
impl ResumeSchemaBasicsLocation {
    pub fn builder() -> builder::ResumeSchemaBasicsLocation {
        Default::default()
    }
}
#[doc = "ResumeSchemaBasicsProfilesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"network\": {"]
#[doc = "      \"description\": \"e.g. Facebook or Twitter\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"e.g. http://twitter.example.com/neutralthoughts\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"username\": {"]
#[doc = "      \"description\": \"e.g. neutralthoughts\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaBasicsProfilesItem {
    #[doc = "e.g. Facebook or Twitter"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub network: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. http://twitter.example.com/neutralthoughts"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. neutralthoughts"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaBasicsProfilesItem> for ResumeSchemaBasicsProfilesItem {
    fn from(value: &ResumeSchemaBasicsProfilesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaBasicsProfilesItem {
    fn default() -> Self {
        Self {
            network: Default::default(),
            url: Default::default(),
            username: Default::default(),
        }
    }
}
impl ResumeSchemaBasicsProfilesItem {
    pub fn builder() -> builder::ResumeSchemaBasicsProfilesItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaCertificatesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"date\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"issuer\": {"]
#[doc = "      \"description\": \"e.g. CNCF\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"e.g. Certified Kubernetes Administrator\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"e.g. http://example.com\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaCertificatesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<Iso8601>,
    #[doc = "e.g. CNCF"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Certified Kubernetes Administrator"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. http://example.com"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaCertificatesItem> for ResumeSchemaCertificatesItem {
    fn from(value: &ResumeSchemaCertificatesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaCertificatesItem {
    fn default() -> Self {
        Self {
            date: Default::default(),
            issuer: Default::default(),
            name: Default::default(),
            url: Default::default(),
        }
    }
}
impl ResumeSchemaCertificatesItem {
    pub fn builder() -> builder::ResumeSchemaCertificatesItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaEducationItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"area\": {"]
#[doc = "      \"description\": \"e.g. Arts\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"courses\": {"]
#[doc = "      \"description\": \"List notable courses/subjects\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. H1302 - Introduction to American history\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"endDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"institution\": {"]
#[doc = "      \"description\": \"e.g. Massachusetts Institute of Technology\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"score\": {"]
#[doc = "      \"description\": \"grade point average, e.g. 3.67/4.0\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"startDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"studyType\": {"]
#[doc = "      \"description\": \"e.g. Bachelor\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"e.g. http://facebook.example.com\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaEducationItem {
    #[doc = "e.g. Arts"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub area: ::std::option::Option<::std::string::String>,
    #[doc = "List notable courses/subjects"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub courses: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_date: ::std::option::Option<Iso8601>,
    #[doc = "e.g. Massachusetts Institute of Technology"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub institution: ::std::option::Option<::std::string::String>,
    #[doc = "grade point average, e.g. 3.67/4.0"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub score: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_date: ::std::option::Option<Iso8601>,
    #[doc = "e.g. Bachelor"]
    #[serde(
        rename = "studyType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub study_type: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. http://facebook.example.com"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaEducationItem> for ResumeSchemaEducationItem {
    fn from(value: &ResumeSchemaEducationItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaEducationItem {
    fn default() -> Self {
        Self {
            area: Default::default(),
            courses: Default::default(),
            end_date: Default::default(),
            institution: Default::default(),
            score: Default::default(),
            start_date: Default::default(),
            study_type: Default::default(),
            url: Default::default(),
        }
    }
}
impl ResumeSchemaEducationItem {
    pub fn builder() -> builder::ResumeSchemaEducationItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaInterestsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"keywords\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. Friedrich Nietzsche\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"e.g. Philosophy\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaInterestsItem {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub keywords: ::std::vec::Vec<::std::string::String>,
    #[doc = "e.g. Philosophy"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaInterestsItem> for ResumeSchemaInterestsItem {
    fn from(value: &ResumeSchemaInterestsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaInterestsItem {
    fn default() -> Self {
        Self {
            keywords: Default::default(),
            name: Default::default(),
        }
    }
}
impl ResumeSchemaInterestsItem {
    pub fn builder() -> builder::ResumeSchemaInterestsItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaLanguagesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"fluency\": {"]
#[doc = "      \"description\": \"e.g. Fluent, Beginner\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"language\": {"]
#[doc = "      \"description\": \"e.g. English, Spanish\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaLanguagesItem {
    #[doc = "e.g. Fluent, Beginner"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fluency: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. English, Spanish"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaLanguagesItem> for ResumeSchemaLanguagesItem {
    fn from(value: &ResumeSchemaLanguagesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaLanguagesItem {
    fn default() -> Self {
        Self {
            fluency: Default::default(),
            language: Default::default(),
        }
    }
}
impl ResumeSchemaLanguagesItem {
    pub fn builder() -> builder::ResumeSchemaLanguagesItem {
        Default::default()
    }
}
#[doc = "The schema version and any other tooling configuration lives here"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The schema version and any other tooling configuration lives here\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"canonical\": {"]
#[doc = "      \"description\": \"URL (as per RFC 3986) to latest version of this document\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"lastModified\": {"]
#[doc = "      \"description\": \"Using ISO 8601 with YYYY-MM-DDThh:mm:ss\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"A version field which follows semver - e.g. v1.0.0\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaMeta {
    #[doc = "URL (as per RFC 3986) to latest version of this document"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub canonical: ::std::option::Option<::std::string::String>,
    #[doc = "Using ISO 8601 with YYYY-MM-DDThh:mm:ss"]
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_modified: ::std::option::Option<::std::string::String>,
    #[doc = "A version field which follows semver - e.g. v1.0.0"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaMeta> for ResumeSchemaMeta {
    fn from(value: &ResumeSchemaMeta) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaMeta {
    fn default() -> Self {
        Self {
            canonical: Default::default(),
            last_modified: Default::default(),
            version: Default::default(),
        }
    }
}
impl ResumeSchemaMeta {
    pub fn builder() -> builder::ResumeSchemaMeta {
        Default::default()
    }
}
#[doc = "ResumeSchemaProjectsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Short summary of project. e.g. Collated works of 2017.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"endDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"entity\": {"]
#[doc = "      \"description\": \"Specify the relevant company/entity affiliations e.g. 'greenpeace', 'corporationXYZ'\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"highlights\": {"]
#[doc = "      \"description\": \"Specify multiple features\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. Directs you close but not quite there\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"keywords\": {"]
#[doc = "      \"description\": \"Specify special elements involved\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. AngularJS\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"e.g. The World Wide Web\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"roles\": {"]
#[doc = "      \"description\": \"Specify your role on this project or in company\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. Team Lead, Speaker, Writer\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"startDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \" e.g. 'volunteering', 'presentation', 'talk', 'application', 'conference'\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"e.g. http://www.computer.org/csdl/mags/co/1996/10/rx069-abs.html\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaProjectsItem {
    #[doc = "Short summary of project. e.g. Collated works of 2017."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_date: ::std::option::Option<Iso8601>,
    #[doc = "Specify the relevant company/entity affiliations e.g. 'greenpeace', 'corporationXYZ'"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub entity: ::std::option::Option<::std::string::String>,
    #[doc = "Specify multiple features"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub highlights: ::std::vec::Vec<::std::string::String>,
    #[doc = "Specify special elements involved"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub keywords: ::std::vec::Vec<::std::string::String>,
    #[doc = "e.g. The World Wide Web"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Specify your role on this project or in company"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub roles: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_date: ::std::option::Option<Iso8601>,
    #[doc = " e.g. 'volunteering', 'presentation', 'talk', 'application', 'conference'"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. http://www.computer.org/csdl/mags/co/1996/10/rx069-abs.html"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaProjectsItem> for ResumeSchemaProjectsItem {
    fn from(value: &ResumeSchemaProjectsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaProjectsItem {
    fn default() -> Self {
        Self {
            description: Default::default(),
            end_date: Default::default(),
            entity: Default::default(),
            highlights: Default::default(),
            keywords: Default::default(),
            name: Default::default(),
            roles: Default::default(),
            start_date: Default::default(),
            type_: Default::default(),
            url: Default::default(),
        }
    }
}
impl ResumeSchemaProjectsItem {
    pub fn builder() -> builder::ResumeSchemaProjectsItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaPublicationsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"e.g. The World Wide Web\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"publisher\": {"]
#[doc = "      \"description\": \"e.g. IEEE, Computer Magazine\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"releaseDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"Short summary of publication. e.g. Discussion of the World Wide Web, HTTP, HTML.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"e.g. http://www.computer.org.example.com/csdl/mags/co/1996/10/rx069-abs.html\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaPublicationsItem {
    #[doc = "e.g. The World Wide Web"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. IEEE, Computer Magazine"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub publisher: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "releaseDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub release_date: ::std::option::Option<Iso8601>,
    #[doc = "Short summary of publication. e.g. Discussion of the World Wide Web, HTTP, HTML."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. http://www.computer.org.example.com/csdl/mags/co/1996/10/rx069-abs.html"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaPublicationsItem> for ResumeSchemaPublicationsItem {
    fn from(value: &ResumeSchemaPublicationsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaPublicationsItem {
    fn default() -> Self {
        Self {
            name: Default::default(),
            publisher: Default::default(),
            release_date: Default::default(),
            summary: Default::default(),
            url: Default::default(),
        }
    }
}
impl ResumeSchemaPublicationsItem {
    pub fn builder() -> builder::ResumeSchemaPublicationsItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaReferencesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"e.g. Timothy Cook\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reference\": {"]
#[doc = "      \"description\": \"e.g. Joe blogs was a great employee, who turned up to work at least once a week. He exceeded my expectations when it came to doing nothing.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaReferencesItem {
    #[doc = "e.g. Timothy Cook"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Joe blogs was a great employee, who turned up to work at least once a week. He exceeded my expectations when it came to doing nothing."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reference: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaReferencesItem> for ResumeSchemaReferencesItem {
    fn from(value: &ResumeSchemaReferencesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaReferencesItem {
    fn default() -> Self {
        Self {
            name: Default::default(),
            reference: Default::default(),
        }
    }
}
impl ResumeSchemaReferencesItem {
    pub fn builder() -> builder::ResumeSchemaReferencesItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaSkillsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"keywords\": {"]
#[doc = "      \"description\": \"List some keywords pertaining to this skill\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. HTML\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"level\": {"]
#[doc = "      \"description\": \"e.g. Master\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"e.g. Web Development\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaSkillsItem {
    #[doc = "List some keywords pertaining to this skill"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub keywords: ::std::vec::Vec<::std::string::String>,
    #[doc = "e.g. Master"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Web Development"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaSkillsItem> for ResumeSchemaSkillsItem {
    fn from(value: &ResumeSchemaSkillsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaSkillsItem {
    fn default() -> Self {
        Self {
            keywords: Default::default(),
            level: Default::default(),
            name: Default::default(),
        }
    }
}
impl ResumeSchemaSkillsItem {
    pub fn builder() -> builder::ResumeSchemaSkillsItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaVolunteerItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"endDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"highlights\": {"]
#[doc = "      \"description\": \"Specify accomplishments and achievements\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. Increased profits by 20% from 2011-2012 through viral advertising\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"organization\": {"]
#[doc = "      \"description\": \"e.g. Facebook\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"position\": {"]
#[doc = "      \"description\": \"e.g. Software Engineer\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"startDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"Give an overview of your responsibilities at the company\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"e.g. http://facebook.example.com\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaVolunteerItem {
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_date: ::std::option::Option<Iso8601>,
    #[doc = "Specify accomplishments and achievements"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub highlights: ::std::vec::Vec<::std::string::String>,
    #[doc = "e.g. Facebook"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Software Engineer"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub position: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_date: ::std::option::Option<Iso8601>,
    #[doc = "Give an overview of your responsibilities at the company"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. http://facebook.example.com"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaVolunteerItem> for ResumeSchemaVolunteerItem {
    fn from(value: &ResumeSchemaVolunteerItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaVolunteerItem {
    fn default() -> Self {
        Self {
            end_date: Default::default(),
            highlights: Default::default(),
            organization: Default::default(),
            position: Default::default(),
            start_date: Default::default(),
            summary: Default::default(),
            url: Default::default(),
        }
    }
}
impl ResumeSchemaVolunteerItem {
    pub fn builder() -> builder::ResumeSchemaVolunteerItem {
        Default::default()
    }
}
#[doc = "ResumeSchemaWorkItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"e.g. Social Media Company\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"endDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"highlights\": {"]
#[doc = "      \"description\": \"Specify multiple accomplishments\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"e.g. Increased profits by 20% from 2011-2012 through viral advertising\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"additionalItems\": false"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"e.g. Menlo Park, CA\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"e.g. Facebook\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"position\": {"]
#[doc = "      \"description\": \"e.g. Software Engineer\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"startDate\": {"]
#[doc = "      \"$ref\": \"#/definitions/iso8601\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"description\": \"Give an overview of your responsibilities at the company\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"e.g. http://facebook.example.com\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResumeSchemaWorkItem {
    #[doc = "e.g. Social Media Company"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "endDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_date: ::std::option::Option<Iso8601>,
    #[doc = "Specify multiple accomplishments"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub highlights: ::std::vec::Vec<::std::string::String>,
    #[doc = "e.g. Menlo Park, CA"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Facebook"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. Software Engineer"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub position: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_date: ::std::option::Option<Iso8601>,
    #[doc = "Give an overview of your responsibilities at the company"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "e.g. http://facebook.example.com"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResumeSchemaWorkItem> for ResumeSchemaWorkItem {
    fn from(value: &ResumeSchemaWorkItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ResumeSchemaWorkItem {
    fn default() -> Self {
        Self {
            description: Default::default(),
            end_date: Default::default(),
            highlights: Default::default(),
            location: Default::default(),
            name: Default::default(),
            position: Default::default(),
            start_date: Default::default(),
            summary: Default::default(),
            url: Default::default(),
        }
    }
}
impl ResumeSchemaWorkItem {
    pub fn builder() -> builder::ResumeSchemaWorkItem {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ResumeSchema {
        awards: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaAwardsItem>,
            ::std::string::String,
        >,
        basics: ::std::result::Result<
            ::std::option::Option<super::ResumeSchemaBasics>,
            ::std::string::String,
        >,
        certificates: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaCertificatesItem>,
            ::std::string::String,
        >,
        education: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaEducationItem>,
            ::std::string::String,
        >,
        interests: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaInterestsItem>,
            ::std::string::String,
        >,
        languages: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaLanguagesItem>,
            ::std::string::String,
        >,
        meta: ::std::result::Result<
            ::std::option::Option<super::ResumeSchemaMeta>,
            ::std::string::String,
        >,
        projects: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaProjectsItem>,
            ::std::string::String,
        >,
        publications: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaPublicationsItem>,
            ::std::string::String,
        >,
        references: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaReferencesItem>,
            ::std::string::String,
        >,
        schema: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        skills: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaSkillsItem>,
            ::std::string::String,
        >,
        volunteer: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaVolunteerItem>,
            ::std::string::String,
        >,
        work: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaWorkItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchema {
        fn default() -> Self {
            Self {
                awards: Ok(Default::default()),
                basics: Ok(Default::default()),
                certificates: Ok(Default::default()),
                education: Ok(Default::default()),
                interests: Ok(Default::default()),
                languages: Ok(Default::default()),
                meta: Ok(Default::default()),
                projects: Ok(Default::default()),
                publications: Ok(Default::default()),
                references: Ok(Default::default()),
                schema: Ok(Default::default()),
                skills: Ok(Default::default()),
                volunteer: Ok(Default::default()),
                work: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchema {
        pub fn awards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaAwardsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.awards = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for awards: {}", e));
            self
        }
        pub fn basics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResumeSchemaBasics>>,
            T::Error: ::std::fmt::Display,
        {
            self.basics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for basics: {}", e));
            self
        }
        pub fn certificates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaCertificatesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificates: {}", e));
            self
        }
        pub fn education<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaEducationItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.education = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for education: {}", e));
            self
        }
        pub fn interests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaInterestsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.interests = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interests: {}", e));
            self
        }
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaLanguagesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for languages: {}", e));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResumeSchemaMeta>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {}", e));
            self
        }
        pub fn projects<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaProjectsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.projects = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for projects: {}", e));
            self
        }
        pub fn publications<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaPublicationsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.publications = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for publications: {}", e));
            self
        }
        pub fn references<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaReferencesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.references = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for references: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
        pub fn skills<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaSkillsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.skills = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skills: {}", e));
            self
        }
        pub fn volunteer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaVolunteerItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.volunteer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for volunteer: {}", e));
            self
        }
        pub fn work<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaWorkItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.work = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for work: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchema> for super::ResumeSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                awards: value.awards?,
                basics: value.basics?,
                certificates: value.certificates?,
                education: value.education?,
                interests: value.interests?,
                languages: value.languages?,
                meta: value.meta?,
                projects: value.projects?,
                publications: value.publications?,
                references: value.references?,
                schema: value.schema?,
                skills: value.skills?,
                volunteer: value.volunteer?,
                work: value.work?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchema> for ResumeSchema {
        fn from(value: super::ResumeSchema) -> Self {
            Self {
                awards: Ok(value.awards),
                basics: Ok(value.basics),
                certificates: Ok(value.certificates),
                education: Ok(value.education),
                interests: Ok(value.interests),
                languages: Ok(value.languages),
                meta: Ok(value.meta),
                projects: Ok(value.projects),
                publications: Ok(value.publications),
                references: Ok(value.references),
                schema: Ok(value.schema),
                skills: Ok(value.skills),
                volunteer: Ok(value.volunteer),
                work: Ok(value.work),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaAwardsItem {
        awarder: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        date: ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaAwardsItem {
        fn default() -> Self {
            Self {
                awarder: Ok(Default::default()),
                date: Ok(Default::default()),
                summary: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaAwardsItem {
        pub fn awarder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.awarder = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for awarder: {}", e));
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaAwardsItem> for super::ResumeSchemaAwardsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaAwardsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                awarder: value.awarder?,
                date: value.date?,
                summary: value.summary?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaAwardsItem> for ResumeSchemaAwardsItem {
        fn from(value: super::ResumeSchemaAwardsItem) -> Self {
            Self {
                awarder: Ok(value.awarder),
                date: Ok(value.date),
                summary: Ok(value.summary),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaBasics {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        image: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        location: ::std::result::Result<
            ::std::option::Option<super::ResumeSchemaBasicsLocation>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        phone: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        profiles: ::std::result::Result<
            ::std::vec::Vec<super::ResumeSchemaBasicsProfilesItem>,
            ::std::string::String,
        >,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaBasics {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                image: Ok(Default::default()),
                label: Ok(Default::default()),
                location: Ok(Default::default()),
                name: Ok(Default::default()),
                phone: Ok(Default::default()),
                profiles: Ok(Default::default()),
                summary: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaBasics {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {}", e));
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResumeSchemaBasicsLocation>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn phone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.phone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phone: {}", e));
            self
        }
        pub fn profiles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResumeSchemaBasicsProfilesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.profiles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profiles: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaBasics> for super::ResumeSchemaBasics {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaBasics,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                image: value.image?,
                label: value.label?,
                location: value.location?,
                name: value.name?,
                phone: value.phone?,
                profiles: value.profiles?,
                summary: value.summary?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaBasics> for ResumeSchemaBasics {
        fn from(value: super::ResumeSchemaBasics) -> Self {
            Self {
                email: Ok(value.email),
                image: Ok(value.image),
                label: Ok(value.label),
                location: Ok(value.location),
                name: Ok(value.name),
                phone: Ok(value.phone),
                profiles: Ok(value.profiles),
                summary: Ok(value.summary),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaBasicsLocation {
        address: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        city: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        country_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        postal_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        region: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaBasicsLocation {
        fn default() -> Self {
            Self {
                address: Ok(Default::default()),
                city: Ok(Default::default()),
                country_code: Ok(Default::default()),
                postal_code: Ok(Default::default()),
                region: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaBasicsLocation {
        pub fn address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for address: {}", e));
            self
        }
        pub fn city<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.city = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for city: {}", e));
            self
        }
        pub fn country_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for country_code: {}", e));
            self
        }
        pub fn postal_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for postal_code: {}", e));
            self
        }
        pub fn region<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.region = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for region: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaBasicsLocation> for super::ResumeSchemaBasicsLocation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaBasicsLocation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                address: value.address?,
                city: value.city?,
                country_code: value.country_code?,
                postal_code: value.postal_code?,
                region: value.region?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaBasicsLocation> for ResumeSchemaBasicsLocation {
        fn from(value: super::ResumeSchemaBasicsLocation) -> Self {
            Self {
                address: Ok(value.address),
                city: Ok(value.city),
                country_code: Ok(value.country_code),
                postal_code: Ok(value.postal_code),
                region: Ok(value.region),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaBasicsProfilesItem {
        network: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        username: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaBasicsProfilesItem {
        fn default() -> Self {
            Self {
                network: Ok(Default::default()),
                url: Ok(Default::default()),
                username: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaBasicsProfilesItem {
        pub fn network<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.network = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn username<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.username = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for username: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaBasicsProfilesItem>
        for super::ResumeSchemaBasicsProfilesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaBasicsProfilesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                network: value.network?,
                url: value.url?,
                username: value.username?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaBasicsProfilesItem>
        for ResumeSchemaBasicsProfilesItem
    {
        fn from(value: super::ResumeSchemaBasicsProfilesItem) -> Self {
            Self {
                network: Ok(value.network),
                url: Ok(value.url),
                username: Ok(value.username),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaCertificatesItem {
        date: ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        issuer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaCertificatesItem {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                issuer: Ok(Default::default()),
                name: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaCertificatesItem {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {}", e));
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaCertificatesItem> for super::ResumeSchemaCertificatesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaCertificatesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                issuer: value.issuer?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaCertificatesItem> for ResumeSchemaCertificatesItem {
        fn from(value: super::ResumeSchemaCertificatesItem) -> Self {
            Self {
                date: Ok(value.date),
                issuer: Ok(value.issuer),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaEducationItem {
        area: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        courses:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        end_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        institution: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        score: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        start_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        study_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaEducationItem {
        fn default() -> Self {
            Self {
                area: Ok(Default::default()),
                courses: Ok(Default::default()),
                end_date: Ok(Default::default()),
                institution: Ok(Default::default()),
                score: Ok(Default::default()),
                start_date: Ok(Default::default()),
                study_type: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaEducationItem {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn courses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.courses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for courses: {}", e));
            self
        }
        pub fn end_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_date: {}", e));
            self
        }
        pub fn institution<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.institution = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for institution: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn start_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_date: {}", e));
            self
        }
        pub fn study_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.study_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for study_type: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaEducationItem> for super::ResumeSchemaEducationItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaEducationItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                courses: value.courses?,
                end_date: value.end_date?,
                institution: value.institution?,
                score: value.score?,
                start_date: value.start_date?,
                study_type: value.study_type?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaEducationItem> for ResumeSchemaEducationItem {
        fn from(value: super::ResumeSchemaEducationItem) -> Self {
            Self {
                area: Ok(value.area),
                courses: Ok(value.courses),
                end_date: Ok(value.end_date),
                institution: Ok(value.institution),
                score: Ok(value.score),
                start_date: Ok(value.start_date),
                study_type: Ok(value.study_type),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaInterestsItem {
        keywords:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaInterestsItem {
        fn default() -> Self {
            Self {
                keywords: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaInterestsItem {
        pub fn keywords<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.keywords = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keywords: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaInterestsItem> for super::ResumeSchemaInterestsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaInterestsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                keywords: value.keywords?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaInterestsItem> for ResumeSchemaInterestsItem {
        fn from(value: super::ResumeSchemaInterestsItem) -> Self {
            Self {
                keywords: Ok(value.keywords),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaLanguagesItem {
        fluency: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        language: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaLanguagesItem {
        fn default() -> Self {
            Self {
                fluency: Ok(Default::default()),
                language: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaLanguagesItem {
        pub fn fluency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fluency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fluency: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaLanguagesItem> for super::ResumeSchemaLanguagesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaLanguagesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fluency: value.fluency?,
                language: value.language?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaLanguagesItem> for ResumeSchemaLanguagesItem {
        fn from(value: super::ResumeSchemaLanguagesItem) -> Self {
            Self {
                fluency: Ok(value.fluency),
                language: Ok(value.language),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaMeta {
        canonical: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        last_modified: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaMeta {
        fn default() -> Self {
            Self {
                canonical: Ok(Default::default()),
                last_modified: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaMeta {
        pub fn canonical<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.canonical = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for canonical: {}", e));
            self
        }
        pub fn last_modified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_modified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_modified: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaMeta> for super::ResumeSchemaMeta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaMeta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                canonical: value.canonical?,
                last_modified: value.last_modified?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaMeta> for ResumeSchemaMeta {
        fn from(value: super::ResumeSchemaMeta) -> Self {
            Self {
                canonical: Ok(value.canonical),
                last_modified: Ok(value.last_modified),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaProjectsItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        end_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        entity: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        highlights:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        keywords:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        roles: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaProjectsItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                end_date: Ok(Default::default()),
                entity: Ok(Default::default()),
                highlights: Ok(Default::default()),
                keywords: Ok(Default::default()),
                name: Ok(Default::default()),
                roles: Ok(Default::default()),
                start_date: Ok(Default::default()),
                type_: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaProjectsItem {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn end_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_date: {}", e));
            self
        }
        pub fn entity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.entity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity: {}", e));
            self
        }
        pub fn highlights<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.highlights = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for highlights: {}", e));
            self
        }
        pub fn keywords<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.keywords = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keywords: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn roles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.roles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roles: {}", e));
            self
        }
        pub fn start_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_date: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaProjectsItem> for super::ResumeSchemaProjectsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaProjectsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                end_date: value.end_date?,
                entity: value.entity?,
                highlights: value.highlights?,
                keywords: value.keywords?,
                name: value.name?,
                roles: value.roles?,
                start_date: value.start_date?,
                type_: value.type_?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaProjectsItem> for ResumeSchemaProjectsItem {
        fn from(value: super::ResumeSchemaProjectsItem) -> Self {
            Self {
                description: Ok(value.description),
                end_date: Ok(value.end_date),
                entity: Ok(value.entity),
                highlights: Ok(value.highlights),
                keywords: Ok(value.keywords),
                name: Ok(value.name),
                roles: Ok(value.roles),
                start_date: Ok(value.start_date),
                type_: Ok(value.type_),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaPublicationsItem {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        publisher: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        release_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaPublicationsItem {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                publisher: Ok(Default::default()),
                release_date: Ok(Default::default()),
                summary: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaPublicationsItem {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn publisher<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.publisher = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for publisher: {}", e));
            self
        }
        pub fn release_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.release_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for release_date: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaPublicationsItem> for super::ResumeSchemaPublicationsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaPublicationsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                publisher: value.publisher?,
                release_date: value.release_date?,
                summary: value.summary?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaPublicationsItem> for ResumeSchemaPublicationsItem {
        fn from(value: super::ResumeSchemaPublicationsItem) -> Self {
            Self {
                name: Ok(value.name),
                publisher: Ok(value.publisher),
                release_date: Ok(value.release_date),
                summary: Ok(value.summary),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaReferencesItem {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reference: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaReferencesItem {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                reference: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaReferencesItem {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaReferencesItem> for super::ResumeSchemaReferencesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaReferencesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                reference: value.reference?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaReferencesItem> for ResumeSchemaReferencesItem {
        fn from(value: super::ResumeSchemaReferencesItem) -> Self {
            Self {
                name: Ok(value.name),
                reference: Ok(value.reference),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaSkillsItem {
        keywords:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        level: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaSkillsItem {
        fn default() -> Self {
            Self {
                keywords: Ok(Default::default()),
                level: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaSkillsItem {
        pub fn keywords<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.keywords = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keywords: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaSkillsItem> for super::ResumeSchemaSkillsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaSkillsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                keywords: value.keywords?,
                level: value.level?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaSkillsItem> for ResumeSchemaSkillsItem {
        fn from(value: super::ResumeSchemaSkillsItem) -> Self {
            Self {
                keywords: Ok(value.keywords),
                level: Ok(value.level),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaVolunteerItem {
        end_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        highlights:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        organization: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        position: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        start_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaVolunteerItem {
        fn default() -> Self {
            Self {
                end_date: Ok(Default::default()),
                highlights: Ok(Default::default()),
                organization: Ok(Default::default()),
                position: Ok(Default::default()),
                start_date: Ok(Default::default()),
                summary: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaVolunteerItem {
        pub fn end_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_date: {}", e));
            self
        }
        pub fn highlights<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.highlights = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for highlights: {}", e));
            self
        }
        pub fn organization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.organization = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for organization: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn start_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_date: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaVolunteerItem> for super::ResumeSchemaVolunteerItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaVolunteerItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_date: value.end_date?,
                highlights: value.highlights?,
                organization: value.organization?,
                position: value.position?,
                start_date: value.start_date?,
                summary: value.summary?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaVolunteerItem> for ResumeSchemaVolunteerItem {
        fn from(value: super::ResumeSchemaVolunteerItem) -> Self {
            Self {
                end_date: Ok(value.end_date),
                highlights: Ok(value.highlights),
                organization: Ok(value.organization),
                position: Ok(value.position),
                start_date: Ok(value.start_date),
                summary: Ok(value.summary),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResumeSchemaWorkItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        end_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        highlights:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        location: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        position: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        start_date:
            ::std::result::Result<::std::option::Option<super::Iso8601>, ::std::string::String>,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ResumeSchemaWorkItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                end_date: Ok(Default::default()),
                highlights: Ok(Default::default()),
                location: Ok(Default::default()),
                name: Ok(Default::default()),
                position: Ok(Default::default()),
                start_date: Ok(Default::default()),
                summary: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl ResumeSchemaWorkItem {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn end_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_date: {}", e));
            self
        }
        pub fn highlights<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.highlights = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for highlights: {}", e));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn start_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Iso8601>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_date: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ResumeSchemaWorkItem> for super::ResumeSchemaWorkItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResumeSchemaWorkItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                end_date: value.end_date?,
                highlights: value.highlights?,
                location: value.location?,
                name: value.name?,
                position: value.position?,
                start_date: value.start_date?,
                summary: value.summary?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ResumeSchemaWorkItem> for ResumeSchemaWorkItem {
        fn from(value: super::ResumeSchemaWorkItem) -> Self {
            Self {
                description: Ok(value.description),
                end_date: Ok(value.end_date),
                highlights: Ok(value.highlights),
                location: Ok(value.location),
                name: Ok(value.name),
                position: Ok(value.position),
                start_date: Ok(value.start_date),
                summary: Ok(value.summary),
                url: Ok(value.url),
            }
        }
    }
}
