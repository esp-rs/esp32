#[doc = "Reader of register HOST_SLCHOST_CONF_W15_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W15_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W15_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W15_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W15_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W15_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF63`"]
pub type HOST_SLCHOST_CONF63_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF63`"]
pub struct HOST_SLCHOST_CONF63_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF63_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF62`"]
pub type HOST_SLCHOST_CONF62_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF62`"]
pub struct HOST_SLCHOST_CONF62_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF62_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF61`"]
pub type HOST_SLCHOST_CONF61_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF61`"]
pub struct HOST_SLCHOST_CONF61_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF61_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF60`"]
pub type HOST_SLCHOST_CONF60_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF60`"]
pub struct HOST_SLCHOST_CONF60_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF60_W<'a> {
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
    pub fn host_slchost_conf63(&self) -> HOST_SLCHOST_CONF63_R {
        HOST_SLCHOST_CONF63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf62(&self) -> HOST_SLCHOST_CONF62_R {
        HOST_SLCHOST_CONF62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf61(&self) -> HOST_SLCHOST_CONF61_R {
        HOST_SLCHOST_CONF61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf60(&self) -> HOST_SLCHOST_CONF60_R {
        HOST_SLCHOST_CONF60_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf63(&mut self) -> HOST_SLCHOST_CONF63_W {
        HOST_SLCHOST_CONF63_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf62(&mut self) -> HOST_SLCHOST_CONF62_W {
        HOST_SLCHOST_CONF62_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf61(&mut self) -> HOST_SLCHOST_CONF61_W {
        HOST_SLCHOST_CONF61_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf60(&mut self) -> HOST_SLCHOST_CONF60_W {
        HOST_SLCHOST_CONF60_W { w: self }
    }
}
