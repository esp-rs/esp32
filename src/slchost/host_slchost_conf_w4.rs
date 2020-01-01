#[doc = "Reader of register HOST_SLCHOST_CONF_W4"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W4>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W4"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W4>;
#[doc = "Register HOST_SLCHOST_CONF_W4 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF19`"]
pub type HOST_SLCHOST_CONF19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF19`"]
pub struct HOST_SLCHOST_CONF19_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF18`"]
pub type HOST_SLCHOST_CONF18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF18`"]
pub struct HOST_SLCHOST_CONF18_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF17`"]
pub type HOST_SLCHOST_CONF17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF17`"]
pub struct HOST_SLCHOST_CONF17_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF16`"]
pub type HOST_SLCHOST_CONF16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF16`"]
pub struct HOST_SLCHOST_CONF16_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    pub fn host_slchost_conf19(&self) -> HOST_SLCHOST_CONF19_R {
        HOST_SLCHOST_CONF19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf18(&self) -> HOST_SLCHOST_CONF18_R {
        HOST_SLCHOST_CONF18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    pub fn host_slchost_conf17(&self) -> HOST_SLCHOST_CONF17_R {
        HOST_SLCHOST_CONF17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    pub fn host_slchost_conf16(&self) -> HOST_SLCHOST_CONF16_R {
        HOST_SLCHOST_CONF16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    pub fn host_slchost_conf19(&mut self) -> HOST_SLCHOST_CONF19_W {
        HOST_SLCHOST_CONF19_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf18(&mut self) -> HOST_SLCHOST_CONF18_W {
        HOST_SLCHOST_CONF18_W { w: self }
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    pub fn host_slchost_conf17(&mut self) -> HOST_SLCHOST_CONF17_W {
        HOST_SLCHOST_CONF17_W { w: self }
    }
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    pub fn host_slchost_conf16(&mut self) -> HOST_SLCHOST_CONF16_W {
        HOST_SLCHOST_CONF16_W { w: self }
    }
}
