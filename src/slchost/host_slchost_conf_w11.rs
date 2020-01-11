#[doc = "Reader of register HOST_SLCHOST_CONF_W11"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W11>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W11"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W11>;
#[doc = "Register HOST_SLCHOST_CONF_W11 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF47`"]
pub type HOST_SLCHOST_CONF47_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF47`"]
pub struct HOST_SLCHOST_CONF47_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF47_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF46`"]
pub type HOST_SLCHOST_CONF46_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF46`"]
pub struct HOST_SLCHOST_CONF46_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF46_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF45`"]
pub type HOST_SLCHOST_CONF45_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF45`"]
pub struct HOST_SLCHOST_CONF45_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF44`"]
pub type HOST_SLCHOST_CONF44_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF44`"]
pub struct HOST_SLCHOST_CONF44_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF44_W<'a> {
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
    pub fn host_slchost_conf47(&self) -> HOST_SLCHOST_CONF47_R {
        HOST_SLCHOST_CONF47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf46(&self) -> HOST_SLCHOST_CONF46_R {
        HOST_SLCHOST_CONF46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf45(&self) -> HOST_SLCHOST_CONF45_R {
        HOST_SLCHOST_CONF45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf44(&self) -> HOST_SLCHOST_CONF44_R {
        HOST_SLCHOST_CONF44_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf47(&mut self) -> HOST_SLCHOST_CONF47_W {
        HOST_SLCHOST_CONF47_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf46(&mut self) -> HOST_SLCHOST_CONF46_W {
        HOST_SLCHOST_CONF46_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf45(&mut self) -> HOST_SLCHOST_CONF45_W {
        HOST_SLCHOST_CONF45_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf44(&mut self) -> HOST_SLCHOST_CONF44_W {
        HOST_SLCHOST_CONF44_W { w: self }
    }
}
