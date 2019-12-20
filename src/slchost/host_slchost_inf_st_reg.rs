#[doc = "Reader of register HOST_SLCHOST_INF_ST_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_INF_ST_REG>;
#[doc = "Writer for register HOST_SLCHOST_INF_ST_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_INF_ST_REG>;
#[doc = "Register HOST_SLCHOST_INF_ST_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_INF_ST_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SDIO_QUICK_IN`"]
pub type HOST_SDIO_QUICK_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SDIO_QUICK_IN`"]
pub struct HOST_SDIO_QUICK_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SDIO_QUICK_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `HOST_SDIO_NEG_SAMP`"]
pub type HOST_SDIO_NEG_SAMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SDIO_NEG_SAMP`"]
pub struct HOST_SDIO_NEG_SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SDIO_NEG_SAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `HOST_SDIO20_MODE`"]
pub type HOST_SDIO20_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_SDIO20_MODE`"]
pub struct HOST_SDIO20_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SDIO20_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_sdio_quick_in(&self) -> HOST_SDIO_QUICK_IN_R {
        HOST_SDIO_QUICK_IN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_sdio_neg_samp(&self) -> HOST_SDIO_NEG_SAMP_R {
        HOST_SDIO_NEG_SAMP_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_sdio20_mode(&self) -> HOST_SDIO20_MODE_R {
        HOST_SDIO20_MODE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_sdio_quick_in(&mut self) -> HOST_SDIO_QUICK_IN_W {
        HOST_SDIO_QUICK_IN_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_sdio_neg_samp(&mut self) -> HOST_SDIO_NEG_SAMP_W {
        HOST_SDIO_NEG_SAMP_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_sdio20_mode(&mut self) -> HOST_SDIO20_MODE_W {
        HOST_SDIO20_MODE_W { w: self }
    }
}
