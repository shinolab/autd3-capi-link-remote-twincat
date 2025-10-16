#![allow(clippy::missing_safety_doc)]

use std::{
    convert::Infallible,
    ffi::{CStr, c_char},
};

use autd3capi_driver::{core::link::LinkError, *};

use autd3_link_twincat::remote::*;

#[repr(C)]
pub struct Timeouts {
    pub connect: OptionDuration,
    pub read: OptionDuration,
    pub write: OptionDuration,
}

impl From<Timeouts> for autd3_link_twincat::remote::Timeouts {
    fn from(t: Timeouts) -> Self {
        Self {
            connect: t.connect.into(),
            read: t.read.into(),
            write: t.write.into(),
        }
    }
}

#[repr(u8)]
pub enum SourceTag {
    Auto,
    Addr,
    Request,
}

#[repr(C)]
pub struct Source {
    tag: SourceTag,
    addr: *const c_char,
}

impl From<Source> for autd3_link_twincat::remote::Source {
    fn from(s: Source) -> Self {
        match s.tag {
            SourceTag::Auto => Self::Auto,
            SourceTag::Addr => {
                let c_str = unsafe { CStr::from_ptr(s.addr) };
                let str_slice = c_str.to_str().unwrap_or_default();
                let addr = str_slice.parse().unwrap_or_default();
                Self::Addr(addr)
            }
            SourceTag::Request => Self::Request,
        }
    }
}

#[repr(C)]
pub struct RemoteTwinCATOption {
    timeouts: Timeouts,
    source: Source,
}

impl From<RemoteTwinCATOption> for autd3_link_twincat::remote::RemoteTwinCATOption {
    fn from(o: RemoteTwinCATOption) -> Self {
        Self {
            timeouts: o.timeouts.into(),
            source: o.source.into(),
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkRemoteTwinCAT(
    addr: *const c_char,
    ams_net_id: *const c_char,
    option: RemoteTwinCATOption,
) -> ResultLink {
    let addr: std::net::IpAddr = match validate_cstr!(addr, LinkPtr, ResultLink).parse() {
        Ok(addr) => addr,
        Err(e) => return Err::<LinkPtr, _>(e).into(),
    };
    let ams_net_id: AmsNetId = match validate_cstr!(ams_net_id, LinkPtr, ResultLink)
        .to_owned()
        .parse()
    {
        Ok(id) => id,
        Err(e) => return Err::<LinkPtr, _>(LinkError::new(e)).into(),
    };
    Ok::<LinkPtr, Infallible>(RemoteTwinCAT::new(addr, ams_net_id, option.into()).into()).into()
}
