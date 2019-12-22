#[doc = "Reader of register HOST_SLCHOST_CONF_W9_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W9_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W9_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W9_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W9_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W9_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF39`"]
pub type HOST_SLCHOST_CONF39_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF39`"]
pub struct HOST_SLCHOST_CONF39_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF38`"]
pub type HOST_SLCHOST_CONF38_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF38`"]
pub struct HOST_SLCHOST_CONF38_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF37`"]
pub type HOST_SLCHOST_CONF37_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF37`"]
pub struct HOST_SLCHOST_CONF37_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF36`"]
pub type HOST_SLCHOST_CONF36_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF36`"]
pub struct HOST_SLCHOST_CONF36_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF36_W<'a> {
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
    pub fn host_slchost_conf39(&self) -> HOST_SLCHOST_CONF39_R {
        HOST_SLCHOST_CONF39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf38(&self) -> HOST_SLCHOST_CONF38_R {
        HOST_SLCHOST_CONF38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf37(&self) -> HOST_SLCHOST_CONF37_R {
        HOST_SLCHOST_CONF37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf36(&self) -> HOST_SLCHOST_CONF36_R {
        HOST_SLCHOST_CONF36_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf39(&mut self) -> HOST_SLCHOST_CONF39_W {
        HOST_SLCHOST_CONF39_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf38(&mut self) -> HOST_SLCHOST_CONF38_W {
        HOST_SLCHOST_CONF38_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf37(&mut self) -> HOST_SLCHOST_CONF37_W {
        HOST_SLCHOST_CONF37_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf36(&mut self) -> HOST_SLCHOST_CONF36_W {
        HOST_SLCHOST_CONF36_W { w: self }
    }
}
