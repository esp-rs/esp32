#[doc = "Reader of register CFG_DATA7"]
pub type R = crate::R<u32, super::CFG_DATA7>;
#[doc = "Writer for register CFG_DATA7"]
pub type W = crate::W<u32, super::CFG_DATA7>;
#[doc = "Register CFG_DATA7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG_DATA7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HINF_SDIO_IOREADY0`"]
pub type HINF_SDIO_IOREADY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_SDIO_IOREADY0`"]
pub struct HINF_SDIO_IOREADY0_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_IOREADY0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `HINF_SDIO_RST`"]
pub type HINF_SDIO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_SDIO_RST`"]
pub struct HINF_SDIO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `HINF_CHIP_STATE`"]
pub type HINF_CHIP_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HINF_CHIP_STATE`"]
pub struct HINF_CHIP_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_CHIP_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HINF_PIN_STATE`"]
pub type HINF_PIN_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HINF_PIN_STATE`"]
pub struct HINF_PIN_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_PIN_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hinf_sdio_ioready0(&self) -> HINF_SDIO_IOREADY0_R {
        HINF_SDIO_IOREADY0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hinf_sdio_rst(&self) -> HINF_SDIO_RST_R {
        HINF_SDIO_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn hinf_chip_state(&self) -> HINF_CHIP_STATE_R {
        HINF_CHIP_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hinf_pin_state(&self) -> HINF_PIN_STATE_R {
        HINF_PIN_STATE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hinf_sdio_ioready0(&mut self) -> HINF_SDIO_IOREADY0_W {
        HINF_SDIO_IOREADY0_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hinf_sdio_rst(&mut self) -> HINF_SDIO_RST_W {
        HINF_SDIO_RST_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn hinf_chip_state(&mut self) -> HINF_CHIP_STATE_W {
        HINF_CHIP_STATE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hinf_pin_state(&mut self) -> HINF_PIN_STATE_W {
        HINF_PIN_STATE_W { w: self }
    }
}
