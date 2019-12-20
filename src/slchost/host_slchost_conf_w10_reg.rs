#[doc = "Reader of register HOST_SLCHOST_CONF_W10_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W10_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W10_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W10_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W10_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W10_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF43`"]
pub type HOST_SLCHOST_CONF43_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF43`"]
pub struct HOST_SLCHOST_CONF43_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF42`"]
pub type HOST_SLCHOST_CONF42_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF42`"]
pub struct HOST_SLCHOST_CONF42_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF42_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF41`"]
pub type HOST_SLCHOST_CONF41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF41`"]
pub struct HOST_SLCHOST_CONF41_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF40`"]
pub type HOST_SLCHOST_CONF40_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF40`"]
pub struct HOST_SLCHOST_CONF40_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF40_W<'a> {
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
    pub fn host_slchost_conf43(&self) -> HOST_SLCHOST_CONF43_R {
        HOST_SLCHOST_CONF43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf42(&self) -> HOST_SLCHOST_CONF42_R {
        HOST_SLCHOST_CONF42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf41(&self) -> HOST_SLCHOST_CONF41_R {
        HOST_SLCHOST_CONF41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf40(&self) -> HOST_SLCHOST_CONF40_R {
        HOST_SLCHOST_CONF40_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf43(&mut self) -> HOST_SLCHOST_CONF43_W {
        HOST_SLCHOST_CONF43_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf42(&mut self) -> HOST_SLCHOST_CONF42_W {
        HOST_SLCHOST_CONF42_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf41(&mut self) -> HOST_SLCHOST_CONF41_W {
        HOST_SLCHOST_CONF41_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf40(&mut self) -> HOST_SLCHOST_CONF40_W {
        HOST_SLCHOST_CONF40_W { w: self }
    }
}
