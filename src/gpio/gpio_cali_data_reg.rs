#[doc = "Reader of register GPIO_cali_data_REG"]
pub type R = crate::R<u32, super::GPIO_CALI_DATA_REG>;
#[doc = "Writer for register GPIO_cali_data_REG"]
pub type W = crate::W<u32, super::GPIO_CALI_DATA_REG>;
#[doc = "Register GPIO_cali_data_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CALI_DATA_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_CALI_RDY_SYNC2`"]
pub type GPIO_CALI_RDY_SYNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_CALI_RDY_SYNC2`"]
pub struct GPIO_CALI_RDY_SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CALI_RDY_SYNC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `GPIO_CALI_RDY_REAL`"]
pub type GPIO_CALI_RDY_REAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_CALI_RDY_REAL`"]
pub struct GPIO_CALI_RDY_REAL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CALI_RDY_REAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `GPIO_CALI_VALUE_SYNC2`"]
pub type GPIO_CALI_VALUE_SYNC2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_CALI_VALUE_SYNC2`"]
pub struct GPIO_CALI_VALUE_SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CALI_VALUE_SYNC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_cali_rdy_sync2(&self) -> GPIO_CALI_RDY_SYNC2_R {
        GPIO_CALI_RDY_SYNC2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_cali_rdy_real(&self) -> GPIO_CALI_RDY_REAL_R {
        GPIO_CALI_RDY_REAL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn gpio_cali_value_sync2(&self) -> GPIO_CALI_VALUE_SYNC2_R {
        GPIO_CALI_VALUE_SYNC2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_cali_rdy_sync2(&mut self) -> GPIO_CALI_RDY_SYNC2_W {
        GPIO_CALI_RDY_SYNC2_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_cali_rdy_real(&mut self) -> GPIO_CALI_RDY_REAL_W {
        GPIO_CALI_RDY_REAL_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn gpio_cali_value_sync2(&mut self) -> GPIO_CALI_VALUE_SYNC2_W {
        GPIO_CALI_VALUE_SYNC2_W { w: self }
    }
}
