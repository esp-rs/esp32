#[doc = "Reader of register CONF2"]
pub type R = crate::R<u32, super::CONF2>;
#[doc = "Writer for register CONF2"]
pub type W = crate::W<u32, super::CONF2>;
#[doc = "Register CONF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTER_VALID_EN`"]
pub type INTER_VALID_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTER_VALID_EN`"]
pub struct INTER_VALID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_VALID_EN_W<'a> {
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
#[doc = "Reader of field `EXT_ADC_START_EN`"]
pub type EXT_ADC_START_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT_ADC_START_EN`"]
pub struct EXT_ADC_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ADC_START_EN_W<'a> {
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
#[doc = "Reader of field `LCD_EN`"]
pub type LCD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCD_EN`"]
pub struct LCD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_EN_W<'a> {
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
#[doc = "Reader of field `DATA_ENABLE`"]
pub type DATA_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_ENABLE`"]
pub struct DATA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ENABLE_W<'a> {
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
#[doc = "Reader of field `DATA_ENABLE_TEST_EN`"]
pub type DATA_ENABLE_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_ENABLE_TEST_EN`"]
pub struct DATA_ENABLE_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ENABLE_TEST_EN_W<'a> {
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
#[doc = "Reader of field `LCD_TX_SDX2_EN`"]
pub type LCD_TX_SDX2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCD_TX_SDX2_EN`"]
pub struct LCD_TX_SDX2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_TX_SDX2_EN_W<'a> {
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
#[doc = "Reader of field `LCD_TX_WRX2_EN`"]
pub type LCD_TX_WRX2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCD_TX_WRX2_EN`"]
pub struct LCD_TX_WRX2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_TX_WRX2_EN_W<'a> {
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
#[doc = "Reader of field `CAMERA_EN`"]
pub type CAMERA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMERA_EN`"]
pub struct CAMERA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMERA_EN_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inter_valid_en(&self) -> INTER_VALID_EN_R {
        INTER_VALID_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ext_adc_start_en(&self) -> EXT_ADC_START_EN_R {
        EXT_ADC_START_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lcd_en(&self) -> LCD_EN_R {
        LCD_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_enable(&self) -> DATA_ENABLE_R {
        DATA_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_enable_test_en(&self) -> DATA_ENABLE_TEST_EN_R {
        DATA_ENABLE_TEST_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&self) -> LCD_TX_SDX2_EN_R {
        LCD_TX_SDX2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&self) -> LCD_TX_WRX2_EN_R {
        LCD_TX_WRX2_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn camera_en(&self) -> CAMERA_EN_R {
        CAMERA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inter_valid_en(&mut self) -> INTER_VALID_EN_W {
        INTER_VALID_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ext_adc_start_en(&mut self) -> EXT_ADC_START_EN_W {
        EXT_ADC_START_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lcd_en(&mut self) -> LCD_EN_W {
        LCD_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_enable(&mut self) -> DATA_ENABLE_W {
        DATA_ENABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_enable_test_en(&mut self) -> DATA_ENABLE_TEST_EN_W {
        DATA_ENABLE_TEST_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&mut self) -> LCD_TX_SDX2_EN_W {
        LCD_TX_SDX2_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&mut self) -> LCD_TX_WRX2_EN_W {
        LCD_TX_WRX2_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn camera_en(&mut self) -> CAMERA_EN_W {
        CAMERA_EN_W { w: self }
    }
}
