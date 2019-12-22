#[doc = "Reader of register HOST_SLCHOST_CONF_W2_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W2_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W2_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W2_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF11`"]
pub type HOST_SLCHOST_CONF11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF11`"]
pub struct HOST_SLCHOST_CONF11_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF10`"]
pub type HOST_SLCHOST_CONF10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF10`"]
pub struct HOST_SLCHOST_CONF10_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF9`"]
pub type HOST_SLCHOST_CONF9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF9`"]
pub struct HOST_SLCHOST_CONF9_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF8`"]
pub type HOST_SLCHOST_CONF8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF8`"]
pub struct HOST_SLCHOST_CONF8_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF8_W<'a> {
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
    pub fn host_slchost_conf11(&self) -> HOST_SLCHOST_CONF11_R {
        HOST_SLCHOST_CONF11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf10(&self) -> HOST_SLCHOST_CONF10_R {
        HOST_SLCHOST_CONF10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf9(&self) -> HOST_SLCHOST_CONF9_R {
        HOST_SLCHOST_CONF9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf8(&self) -> HOST_SLCHOST_CONF8_R {
        HOST_SLCHOST_CONF8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf11(&mut self) -> HOST_SLCHOST_CONF11_W {
        HOST_SLCHOST_CONF11_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf10(&mut self) -> HOST_SLCHOST_CONF10_W {
        HOST_SLCHOST_CONF10_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf9(&mut self) -> HOST_SLCHOST_CONF9_W {
        HOST_SLCHOST_CONF9_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf8(&mut self) -> HOST_SLCHOST_CONF8_W {
        HOST_SLCHOST_CONF8_W { w: self }
    }
}
