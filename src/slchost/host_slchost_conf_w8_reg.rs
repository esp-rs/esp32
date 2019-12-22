#[doc = "Reader of register HOST_SLCHOST_CONF_W8_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W8_REG>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W8_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W8_REG>;
#[doc = "Register HOST_SLCHOST_CONF_W8_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W8_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF35`"]
pub type HOST_SLCHOST_CONF35_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF35`"]
pub struct HOST_SLCHOST_CONF35_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF34`"]
pub type HOST_SLCHOST_CONF34_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF34`"]
pub struct HOST_SLCHOST_CONF34_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF33`"]
pub type HOST_SLCHOST_CONF33_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF33`"]
pub struct HOST_SLCHOST_CONF33_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF32`"]
pub type HOST_SLCHOST_CONF32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF32`"]
pub struct HOST_SLCHOST_CONF32_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF32_W<'a> {
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
    pub fn host_slchost_conf35(&self) -> HOST_SLCHOST_CONF35_R {
        HOST_SLCHOST_CONF35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf34(&self) -> HOST_SLCHOST_CONF34_R {
        HOST_SLCHOST_CONF34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf33(&self) -> HOST_SLCHOST_CONF33_R {
        HOST_SLCHOST_CONF33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf32(&self) -> HOST_SLCHOST_CONF32_R {
        HOST_SLCHOST_CONF32_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf35(&mut self) -> HOST_SLCHOST_CONF35_W {
        HOST_SLCHOST_CONF35_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf34(&mut self) -> HOST_SLCHOST_CONF34_W {
        HOST_SLCHOST_CONF34_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf33(&mut self) -> HOST_SLCHOST_CONF33_W {
        HOST_SLCHOST_CONF33_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf32(&mut self) -> HOST_SLCHOST_CONF32_W {
        HOST_SLCHOST_CONF32_W { w: self }
    }
}
