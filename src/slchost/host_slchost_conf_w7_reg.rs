#[doc = "Reader of register HOST_SLCHOST_CONF_W7_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W7_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W7_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W7_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W7_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W7_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF31`"]
pub type HOST_SLCHOST_CONF31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF31`"]
pub struct HOST_SLCHOST_CONF31_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF30`"]
pub type HOST_SLCHOST_CONF30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF30`"]
pub struct HOST_SLCHOST_CONF30_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF29`"]
pub type HOST_SLCHOST_CONF29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF29`"]
pub struct HOST_SLCHOST_CONF29_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF28`"]
pub type HOST_SLCHOST_CONF28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF28`"]
pub struct HOST_SLCHOST_CONF28_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF28_W<'a> {
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
    pub fn host_slchost_conf31(&self) -> HOST_SLCHOST_CONF31_R {
        HOST_SLCHOST_CONF31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf30(&self) -> HOST_SLCHOST_CONF30_R {
        HOST_SLCHOST_CONF30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf29(&self) -> HOST_SLCHOST_CONF29_R {
        HOST_SLCHOST_CONF29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf28(&self) -> HOST_SLCHOST_CONF28_R {
        HOST_SLCHOST_CONF28_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf31(&mut self) -> HOST_SLCHOST_CONF31_W {
        HOST_SLCHOST_CONF31_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf30(&mut self) -> HOST_SLCHOST_CONF30_W {
        HOST_SLCHOST_CONF30_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf29(&mut self) -> HOST_SLCHOST_CONF29_W {
        HOST_SLCHOST_CONF29_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf28(&mut self) -> HOST_SLCHOST_CONF28_W {
        HOST_SLCHOST_CONF28_W { w: self }
    }
}
