#[doc = "Reader of register I2S_CONF2_REG"]
pub type R = crate::R<u32, super::I2S_CONF2_REG>;
#[doc = "Writer for register I2S_CONF2_REG"]
pub type W = crate::W<u32, super::I2S_CONF2_REG>;
#[doc = "Register I2S_CONF2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CONF2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_INTER_VALID_EN`"]
pub type I2S_INTER_VALID_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_INTER_VALID_EN`"]
pub struct I2S_INTER_VALID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INTER_VALID_EN_W<'a> {
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
#[doc = "Reader of field `I2S_EXT_ADC_START_EN`"]
pub type I2S_EXT_ADC_START_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_EXT_ADC_START_EN`"]
pub struct I2S_EXT_ADC_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_EXT_ADC_START_EN_W<'a> {
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
#[doc = "Reader of field `I2S_LCD_EN`"]
pub type I2S_LCD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_LCD_EN`"]
pub struct I2S_LCD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LCD_EN_W<'a> {
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
#[doc = "Reader of field `I2S_DATA_ENABLE`"]
pub type I2S_DATA_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_DATA_ENABLE`"]
pub struct I2S_DATA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_DATA_ENABLE_W<'a> {
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
#[doc = "Reader of field `I2S_DATA_ENABLE_TEST_EN`"]
pub type I2S_DATA_ENABLE_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_DATA_ENABLE_TEST_EN`"]
pub struct I2S_DATA_ENABLE_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_DATA_ENABLE_TEST_EN_W<'a> {
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
#[doc = "Reader of field `I2S_LCD_TX_SDX2_EN`"]
pub type I2S_LCD_TX_SDX2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_LCD_TX_SDX2_EN`"]
pub struct I2S_LCD_TX_SDX2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LCD_TX_SDX2_EN_W<'a> {
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
#[doc = "Reader of field `I2S_LCD_TX_WRX2_EN`"]
pub type I2S_LCD_TX_WRX2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_LCD_TX_WRX2_EN`"]
pub struct I2S_LCD_TX_WRX2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LCD_TX_WRX2_EN_W<'a> {
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
#[doc = "Reader of field `I2S_CAMERA_EN`"]
pub type I2S_CAMERA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CAMERA_EN`"]
pub struct I2S_CAMERA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CAMERA_EN_W<'a> {
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
    pub fn i2s_inter_valid_en(&self) -> I2S_INTER_VALID_EN_R {
        I2S_INTER_VALID_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_ext_adc_start_en(&self) -> I2S_EXT_ADC_START_EN_R {
        I2S_EXT_ADC_START_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_lcd_en(&self) -> I2S_LCD_EN_R {
        I2S_LCD_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_data_enable(&self) -> I2S_DATA_ENABLE_R {
        I2S_DATA_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_data_enable_test_en(&self) -> I2S_DATA_ENABLE_TEST_EN_R {
        I2S_DATA_ENABLE_TEST_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_lcd_tx_sdx2_en(&self) -> I2S_LCD_TX_SDX2_EN_R {
        I2S_LCD_TX_SDX2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_lcd_tx_wrx2_en(&self) -> I2S_LCD_TX_WRX2_EN_R {
        I2S_LCD_TX_WRX2_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_camera_en(&self) -> I2S_CAMERA_EN_R {
        I2S_CAMERA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_inter_valid_en(&mut self) -> I2S_INTER_VALID_EN_W {
        I2S_INTER_VALID_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_ext_adc_start_en(&mut self) -> I2S_EXT_ADC_START_EN_W {
        I2S_EXT_ADC_START_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_lcd_en(&mut self) -> I2S_LCD_EN_W {
        I2S_LCD_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_data_enable(&mut self) -> I2S_DATA_ENABLE_W {
        I2S_DATA_ENABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_data_enable_test_en(&mut self) -> I2S_DATA_ENABLE_TEST_EN_W {
        I2S_DATA_ENABLE_TEST_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_lcd_tx_sdx2_en(&mut self) -> I2S_LCD_TX_SDX2_EN_W {
        I2S_LCD_TX_SDX2_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_lcd_tx_wrx2_en(&mut self) -> I2S_LCD_TX_WRX2_EN_W {
        I2S_LCD_TX_WRX2_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_camera_en(&mut self) -> I2S_CAMERA_EN_W {
        I2S_CAMERA_EN_W { w: self }
    }
}
