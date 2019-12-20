#[doc = "Reader of register HOST_SLCHOST_CONF_W6_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W6_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W6_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W6_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W6_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W6_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF27`"]
pub type HOST_SLCHOST_CONF27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF27`"]
pub struct HOST_SLCHOST_CONF27_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF26`"]
pub type HOST_SLCHOST_CONF26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF26`"]
pub struct HOST_SLCHOST_CONF26_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF25`"]
pub type HOST_SLCHOST_CONF25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF25`"]
pub struct HOST_SLCHOST_CONF25_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF24`"]
pub type HOST_SLCHOST_CONF24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF24`"]
pub struct HOST_SLCHOST_CONF24_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF24_W<'a> {
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
    pub fn host_slchost_conf27(&self) -> HOST_SLCHOST_CONF27_R {
        HOST_SLCHOST_CONF27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf26(&self) -> HOST_SLCHOST_CONF26_R {
        HOST_SLCHOST_CONF26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf25(&self) -> HOST_SLCHOST_CONF25_R {
        HOST_SLCHOST_CONF25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf24(&self) -> HOST_SLCHOST_CONF24_R {
        HOST_SLCHOST_CONF24_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf27(&mut self) -> HOST_SLCHOST_CONF27_W {
        HOST_SLCHOST_CONF27_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf26(&mut self) -> HOST_SLCHOST_CONF26_W {
        HOST_SLCHOST_CONF26_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf25(&mut self) -> HOST_SLCHOST_CONF25_W {
        HOST_SLCHOST_CONF25_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf24(&mut self) -> HOST_SLCHOST_CONF24_W {
        HOST_SLCHOST_CONF24_W { w: self }
    }
}
