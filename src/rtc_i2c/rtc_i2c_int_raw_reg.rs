#[doc = "Reader of register RTC_I2C_INT_RAW_REG"]
pub type R = crate::R<u32, super::RTC_I2C_INT_RAW_REG>;
#[doc = "Writer for register RTC_I2C_INT_RAW_REG"]
pub type W = crate::W<u32, super::RTC_I2C_INT_RAW_REG>;
#[doc = "Register RTC_I2C_INT_RAW_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_INT_RAW_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_TIME_OUT_INT_RAW`"]
pub type RTC_I2C_TIME_OUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TIME_OUT_INT_RAW`"]
pub struct RTC_I2C_TIME_OUT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TIME_OUT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TRANS_COMPLETE_INT_RAW`"]
pub type RTC_I2C_TRANS_COMPLETE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TRANS_COMPLETE_INT_RAW`"]
pub struct RTC_I2C_TRANS_COMPLETE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TRANS_COMPLETE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW`"]
pub type RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW`"]
pub struct RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_ARBITRATION_LOST_INT_RAW`"]
pub type RTC_I2C_ARBITRATION_LOST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_ARBITRATION_LOST_INT_RAW`"]
pub struct RTC_I2C_ARBITRATION_LOST_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ARBITRATION_LOST_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW`"]
pub type RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW`"]
pub struct RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW_W<'a> {
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
impl R {
    #[doc = "Bit 7 - time out interrupt raw status"]
    #[inline(always)]
    pub fn rtc_i2c_time_out_int_raw(&self) -> RTC_I2C_TIME_OUT_INT_RAW_R {
        RTC_I2C_TIME_OUT_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_raw(&self) -> RTC_I2C_TRANS_COMPLETE_INT_RAW_R {
        RTC_I2C_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_master_trans_complete_int_raw(&self) -> RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW_R {
        RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_raw(&self) -> RTC_I2C_ARBITRATION_LOST_INT_RAW_R {
        RTC_I2C_ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    pub fn rtc_i2c_slave_trans_complete_int_raw(&self) -> RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW_R {
        RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - time out interrupt raw status"]
    #[inline(always)]
    pub fn rtc_i2c_time_out_int_raw(&mut self) -> RTC_I2C_TIME_OUT_INT_RAW_W {
        RTC_I2C_TIME_OUT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    pub fn rtc_i2c_trans_complete_int_raw(&mut self) -> RTC_I2C_TRANS_COMPLETE_INT_RAW_W {
        RTC_I2C_TRANS_COMPLETE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_master_trans_complete_int_raw(
        &mut self,
    ) -> RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW_W {
        RTC_I2C_MASTER_TRANS_COMPLETE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    pub fn rtc_i2c_arbitration_lost_int_raw(&mut self) -> RTC_I2C_ARBITRATION_LOST_INT_RAW_W {
        RTC_I2C_ARBITRATION_LOST_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    pub fn rtc_i2c_slave_trans_complete_int_raw(
        &mut self,
    ) -> RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW_W {
        RTC_I2C_SLAVE_TRANS_COMPLETE_INT_RAW_W { w: self }
    }
}
