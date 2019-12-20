#[doc = "Reader of register HOST_SLCHOST_CONF_W5_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W5_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W5_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W5_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W5_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W5_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF23`"]
pub type HOST_SLCHOST_CONF23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF23`"]
pub struct HOST_SLCHOST_CONF23_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF22`"]
pub type HOST_SLCHOST_CONF22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF22`"]
pub struct HOST_SLCHOST_CONF22_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF21`"]
pub type HOST_SLCHOST_CONF21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF21`"]
pub struct HOST_SLCHOST_CONF21_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF20`"]
pub type HOST_SLCHOST_CONF20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF20`"]
pub struct HOST_SLCHOST_CONF20_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf23(&self) -> HOST_SLCHOST_CONF23_R {
        HOST_SLCHOST_CONF23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf22(&self) -> HOST_SLCHOST_CONF22_R {
        HOST_SLCHOST_CONF22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf21(&self) -> HOST_SLCHOST_CONF21_R {
        HOST_SLCHOST_CONF21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf20(&self) -> HOST_SLCHOST_CONF20_R {
        HOST_SLCHOST_CONF20_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf23(&mut self) -> HOST_SLCHOST_CONF23_W {
        HOST_SLCHOST_CONF23_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf22(&mut self) -> HOST_SLCHOST_CONF22_W {
        HOST_SLCHOST_CONF22_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf21(&mut self) -> HOST_SLCHOST_CONF21_W {
        HOST_SLCHOST_CONF21_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf20(&mut self) -> HOST_SLCHOST_CONF20_W {
        HOST_SLCHOST_CONF20_W { w: self }
    }
}
