#[doc = "Reader of register HOST_SLCHOST_CONF_W14"]
pub type R = crate::R<u32, super::HOST_SLCHOST_CONF_W14>;
#[doc = "Writer for register HOST_SLCHOST_CONF_W14"]
pub type W = crate::W<u32, super::HOST_SLCHOST_CONF_W14>;
#[doc = "Register HOST_SLCHOST_CONF_W14 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_CONF_W14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF59`"]
pub type HOST_SLCHOST_CONF59_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF59`"]
pub struct HOST_SLCHOST_CONF59_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF59_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF58`"]
pub type HOST_SLCHOST_CONF58_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF58`"]
pub struct HOST_SLCHOST_CONF58_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF58_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF57`"]
pub type HOST_SLCHOST_CONF57_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF57`"]
pub struct HOST_SLCHOST_CONF57_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF57_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLCHOST_CONF56`"]
pub type HOST_SLCHOST_CONF56_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SLCHOST_CONF56`"]
pub struct HOST_SLCHOST_CONF56_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF56_W<'a> {
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
    pub fn host_slchost_conf59(&self) -> HOST_SLCHOST_CONF59_R {
        HOST_SLCHOST_CONF59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf58(&self) -> HOST_SLCHOST_CONF58_R {
        HOST_SLCHOST_CONF58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf57(&self) -> HOST_SLCHOST_CONF57_R {
        HOST_SLCHOST_CONF57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf56(&self) -> HOST_SLCHOST_CONF56_R {
        HOST_SLCHOST_CONF56_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf59(&mut self) -> HOST_SLCHOST_CONF59_W {
        HOST_SLCHOST_CONF59_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf58(&mut self) -> HOST_SLCHOST_CONF58_W {
        HOST_SLCHOST_CONF58_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf57(&mut self) -> HOST_SLCHOST_CONF57_W {
        HOST_SLCHOST_CONF57_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf56(&mut self) -> HOST_SLCHOST_CONF56_W {
        HOST_SLCHOST_CONF56_W { w: self }
    }
}
