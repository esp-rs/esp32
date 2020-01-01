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
#[doc = "Reader of field `HINF_SDIO20_CONF1`"]
pub type HINF_SDIO20_CONF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HINF_SDIO20_CONF1`"]
pub struct HINF_SDIO20_CONF1_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO20_CONF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `HINF_FUNC2_EPS`"]
pub type HINF_FUNC2_EPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_FUNC2_EPS`"]
pub struct HINF_FUNC2_EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_FUNC2_EPS_W<'a> {
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
#[doc = "Reader of field `HINF_SDIO_VER`"]
pub type HINF_SDIO_VER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HINF_SDIO_VER`"]
pub struct HINF_SDIO_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HINF_SDIO20_CONF0`"]
pub type HINF_SDIO20_CONF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HINF_SDIO20_CONF0`"]
pub struct HINF_SDIO20_CONF0_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO20_CONF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `HINF_IOENABLE1`"]
pub type HINF_IOENABLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_IOENABLE1`"]
pub struct HINF_IOENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_IOENABLE1_W<'a> {
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
#[doc = "Reader of field `HINF_EMP`"]
pub type HINF_EMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_EMP`"]
pub struct HINF_EMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_EMP_W<'a> {
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
#[doc = "Reader of field `HINF_FUNC1_EPS`"]
pub type HINF_FUNC1_EPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_FUNC1_EPS`"]
pub struct HINF_FUNC1_EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_FUNC1_EPS_W<'a> {
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
#[doc = "Reader of field `HINF_CD_DISABLE`"]
pub type HINF_CD_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_CD_DISABLE`"]
pub struct HINF_CD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_CD_DISABLE_W<'a> {
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
#[doc = "Reader of field `HINF_IOENABLE2`"]
pub type HINF_IOENABLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_IOENABLE2`"]
pub struct HINF_IOENABLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_IOENABLE2_W<'a> {
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
#[doc = "Reader of field `HINF_SDIO_INT_MASK`"]
pub type HINF_SDIO_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_SDIO_INT_MASK`"]
pub struct HINF_SDIO_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_INT_MASK_W<'a> {
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
#[doc = "Reader of field `HINF_SDIO_IOREADY2`"]
pub type HINF_SDIO_IOREADY2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_SDIO_IOREADY2`"]
pub struct HINF_SDIO_IOREADY2_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_IOREADY2_W<'a> {
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
#[doc = "Reader of field `HINF_SDIO_CD_ENABLE`"]
pub type HINF_SDIO_CD_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_SDIO_CD_ENABLE`"]
pub struct HINF_SDIO_CD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_CD_ENABLE_W<'a> {
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
#[doc = "Reader of field `HINF_HIGHSPEED_MODE`"]
pub type HINF_HIGHSPEED_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_HIGHSPEED_MODE`"]
pub struct HINF_HIGHSPEED_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_HIGHSPEED_MODE_W<'a> {
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
#[doc = "Reader of field `HINF_HIGHSPEED_ENABLE`"]
pub type HINF_HIGHSPEED_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_HIGHSPEED_ENABLE`"]
pub struct HINF_HIGHSPEED_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_HIGHSPEED_ENABLE_W<'a> {
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
#[doc = "Reader of field `HINF_SDIO_IOREADY1`"]
pub type HINF_SDIO_IOREADY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_SDIO_IOREADY1`"]
pub struct HINF_SDIO_IOREADY1_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_IOREADY1_W<'a> {
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
#[doc = "Reader of field `HINF_SDIO_ENABLE`"]
pub type HINF_SDIO_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HINF_SDIO_ENABLE`"]
pub struct HINF_SDIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HINF_SDIO_ENABLE_W<'a> {
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
    pub fn hinf_sdio20_conf1(&self) -> HINF_SDIO20_CONF1_R {
        HINF_SDIO20_CONF1_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn hinf_func2_eps(&self) -> HINF_FUNC2_EPS_R {
        HINF_FUNC2_EPS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn hinf_sdio_ver(&self) -> HINF_SDIO_VER_R {
        HINF_SDIO_VER_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hinf_sdio20_conf0(&self) -> HINF_SDIO20_CONF0_R {
        HINF_SDIO20_CONF0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn hinf_ioenable1(&self) -> HINF_IOENABLE1_R {
        HINF_IOENABLE1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn hinf_emp(&self) -> HINF_EMP_R {
        HINF_EMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hinf_func1_eps(&self) -> HINF_FUNC1_EPS_R {
        HINF_FUNC1_EPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hinf_cd_disable(&self) -> HINF_CD_DISABLE_R {
        HINF_CD_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hinf_ioenable2(&self) -> HINF_IOENABLE2_R {
        HINF_IOENABLE2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn hinf_sdio_int_mask(&self) -> HINF_SDIO_INT_MASK_R {
        HINF_SDIO_INT_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hinf_sdio_ioready2(&self) -> HINF_SDIO_IOREADY2_R {
        HINF_SDIO_IOREADY2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hinf_sdio_cd_enable(&self) -> HINF_SDIO_CD_ENABLE_R {
        HINF_SDIO_CD_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hinf_highspeed_mode(&self) -> HINF_HIGHSPEED_MODE_R {
        HINF_HIGHSPEED_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hinf_highspeed_enable(&self) -> HINF_HIGHSPEED_ENABLE_R {
        HINF_HIGHSPEED_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hinf_sdio_ioready1(&self) -> HINF_SDIO_IOREADY1_R {
        HINF_SDIO_IOREADY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hinf_sdio_enable(&self) -> HINF_SDIO_ENABLE_R {
        HINF_SDIO_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn hinf_sdio20_conf1(&mut self) -> HINF_SDIO20_CONF1_W {
        HINF_SDIO20_CONF1_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn hinf_func2_eps(&mut self) -> HINF_FUNC2_EPS_W {
        HINF_FUNC2_EPS_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn hinf_sdio_ver(&mut self) -> HINF_SDIO_VER_W {
        HINF_SDIO_VER_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hinf_sdio20_conf0(&mut self) -> HINF_SDIO20_CONF0_W {
        HINF_SDIO20_CONF0_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn hinf_ioenable1(&mut self) -> HINF_IOENABLE1_W {
        HINF_IOENABLE1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn hinf_emp(&mut self) -> HINF_EMP_W {
        HINF_EMP_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hinf_func1_eps(&mut self) -> HINF_FUNC1_EPS_W {
        HINF_FUNC1_EPS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hinf_cd_disable(&mut self) -> HINF_CD_DISABLE_W {
        HINF_CD_DISABLE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hinf_ioenable2(&mut self) -> HINF_IOENABLE2_W {
        HINF_IOENABLE2_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn hinf_sdio_int_mask(&mut self) -> HINF_SDIO_INT_MASK_W {
        HINF_SDIO_INT_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hinf_sdio_ioready2(&mut self) -> HINF_SDIO_IOREADY2_W {
        HINF_SDIO_IOREADY2_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hinf_sdio_cd_enable(&mut self) -> HINF_SDIO_CD_ENABLE_W {
        HINF_SDIO_CD_ENABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hinf_highspeed_mode(&mut self) -> HINF_HIGHSPEED_MODE_W {
        HINF_HIGHSPEED_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hinf_highspeed_enable(&mut self) -> HINF_HIGHSPEED_ENABLE_W {
        HINF_HIGHSPEED_ENABLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hinf_sdio_ioready1(&mut self) -> HINF_SDIO_IOREADY1_W {
        HINF_SDIO_IOREADY1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hinf_sdio_enable(&mut self) -> HINF_SDIO_ENABLE_W {
        HINF_SDIO_ENABLE_W { w: self }
    }
}
