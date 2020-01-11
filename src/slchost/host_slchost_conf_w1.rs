#[doc = "Reader of register HOST_SLCHOST_CONF_W1"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W1>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W1"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W1>;
#[doc = "Register HOST_SLCHOST_CONF_W1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF7`"]
pub type HOST_SLCHOST_CONF7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF7`"]
pub struct HOST_SLCHOST_CONF7_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF6`"]
pub type HOST_SLCHOST_CONF6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF6`"]
pub struct HOST_SLCHOST_CONF6_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF5`"]
pub type HOST_SLCHOST_CONF5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF5`"]
pub struct HOST_SLCHOST_CONF5_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF4`"]
pub type HOST_SLCHOST_CONF4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF4`"]
pub struct HOST_SLCHOST_CONF4_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF4_W<'a> {
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
    pub fn host_slchost_conf7(&self) -> HOST_SLCHOST_CONF7_R {
        HOST_SLCHOST_CONF7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf6(&self) -> HOST_SLCHOST_CONF6_R {
        HOST_SLCHOST_CONF6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf5(&self) -> HOST_SLCHOST_CONF5_R {
        HOST_SLCHOST_CONF5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf4(&self) -> HOST_SLCHOST_CONF4_R {
        HOST_SLCHOST_CONF4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf7(&mut self) -> HOST_SLCHOST_CONF7_W {
        HOST_SLCHOST_CONF7_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf6(&mut self) -> HOST_SLCHOST_CONF6_W {
        HOST_SLCHOST_CONF6_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf5(&mut self) -> HOST_SLCHOST_CONF5_W {
        HOST_SLCHOST_CONF5_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf4(&mut self) -> HOST_SLCHOST_CONF4_W {
        HOST_SLCHOST_CONF4_W { w: self }
    }
}
