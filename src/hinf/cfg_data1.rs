#[doc = "Reader of register CFG_DATA1"]
pub type R = crate::R<u32, super::CFG_DATA1>;
#[doc = "Writer for register CFG_DATA1"]
pub type W = crate::W<u32, super::CFG_DATA1>;
#[doc = "Register CFG_DATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG_DATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDIO20_CONF1`"]
pub type SDIO20_CONF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO20_CONF1`"]
pub struct SDIO20_CONF1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO20_CONF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `FUNC2_EPS`"]
pub type FUNC2_EPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUNC2_EPS`"]
pub struct FUNC2_EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC2_EPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SDIO_VER`"]
pub type SDIO_VER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDIO_VER`"]
pub struct SDIO_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDIO20_CONF0`"]
pub type SDIO20_CONF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDIO20_CONF0`"]
pub struct SDIO20_CONF0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO20_CONF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IOENABLE1`"]
pub type IOENABLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOENABLE1`"]
pub struct IOENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IOENABLE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `EMP`"]
pub type EMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMP`"]
pub struct EMP_W<'a> {
    w: &'a mut W,
}
impl<'a> EMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FUNC1_EPS`"]
pub type FUNC1_EPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUNC1_EPS`"]
pub struct FUNC1_EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC1_EPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CD_DISABLE`"]
pub type CD_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CD_DISABLE`"]
pub struct CD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `IOENABLE2`"]
pub type IOENABLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOENABLE2`"]
pub struct IOENABLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> IOENABLE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SDIO_INT_MASK`"]
pub type SDIO_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_INT_MASK`"]
pub struct SDIO_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SDIO_IOREADY2`"]
pub type SDIO_IOREADY2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_IOREADY2`"]
pub struct SDIO_IOREADY2_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IOREADY2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SDIO_CD_ENABLE`"]
pub type SDIO_CD_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_CD_ENABLE`"]
pub struct SDIO_CD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_CD_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `HIGHSPEED_MODE`"]
pub type HIGHSPEED_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGHSPEED_MODE`"]
pub struct HIGHSPEED_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHSPEED_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `HIGHSPEED_ENABLE`"]
pub type HIGHSPEED_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGHSPEED_ENABLE`"]
pub struct HIGHSPEED_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHSPEED_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SDIO_IOREADY1`"]
pub type SDIO_IOREADY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_IOREADY1`"]
pub struct SDIO_IOREADY1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IOREADY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SDIO_ENABLE`"]
pub type SDIO_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_ENABLE`"]
pub struct SDIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sdio20_conf1(&self) -> SDIO20_CONF1_R {
        SDIO20_CONF1_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn func2_eps(&self) -> FUNC2_EPS_R {
        FUNC2_EPS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn sdio_ver(&self) -> SDIO_VER_R {
        SDIO_VER_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sdio20_conf0(&self) -> SDIO20_CONF0_R {
        SDIO20_CONF0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ioenable1(&self) -> IOENABLE1_R {
        IOENABLE1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn emp(&self) -> EMP_R {
        EMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn func1_eps(&self) -> FUNC1_EPS_R {
        FUNC1_EPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cd_disable(&self) -> CD_DISABLE_R {
        CD_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ioenable2(&self) -> IOENABLE2_R {
        IOENABLE2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SDIO_INT_MASK_R {
        SDIO_INT_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdio_ioready2(&self) -> SDIO_IOREADY2_R {
        SDIO_IOREADY2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdio_cd_enable(&self) -> SDIO_CD_ENABLE_R {
        SDIO_CD_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn highspeed_mode(&self) -> HIGHSPEED_MODE_R {
        HIGHSPEED_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn highspeed_enable(&self) -> HIGHSPEED_ENABLE_R {
        HIGHSPEED_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdio_ioready1(&self) -> SDIO_IOREADY1_R {
        SDIO_IOREADY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdio_enable(&self) -> SDIO_ENABLE_R {
        SDIO_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sdio20_conf1(&mut self) -> SDIO20_CONF1_W {
        SDIO20_CONF1_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn func2_eps(&mut self) -> FUNC2_EPS_W {
        FUNC2_EPS_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn sdio_ver(&mut self) -> SDIO_VER_W {
        SDIO_VER_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sdio20_conf0(&mut self) -> SDIO20_CONF0_W {
        SDIO20_CONF0_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ioenable1(&mut self) -> IOENABLE1_W {
        IOENABLE1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn emp(&mut self) -> EMP_W {
        EMP_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn func1_eps(&mut self) -> FUNC1_EPS_W {
        FUNC1_EPS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cd_disable(&mut self) -> CD_DISABLE_W {
        CD_DISABLE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ioenable2(&mut self) -> IOENABLE2_W {
        IOENABLE2_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W {
        SDIO_INT_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdio_ioready2(&mut self) -> SDIO_IOREADY2_W {
        SDIO_IOREADY2_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdio_cd_enable(&mut self) -> SDIO_CD_ENABLE_W {
        SDIO_CD_ENABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn highspeed_mode(&mut self) -> HIGHSPEED_MODE_W {
        HIGHSPEED_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn highspeed_enable(&mut self) -> HIGHSPEED_ENABLE_W {
        HIGHSPEED_ENABLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdio_ioready1(&mut self) -> SDIO_IOREADY1_W {
        SDIO_IOREADY1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdio_enable(&mut self) -> SDIO_ENABLE_W {
        SDIO_ENABLE_W { w: self }
    }
}
