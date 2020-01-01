#[doc = "Reader of register DEBUG_STATUS"]
pub type R = crate::R<u32, super::DEBUG_STATUS>;
#[doc = "Writer for register DEBUG_STATUS"]
pub type W = crate::W<u32, super::DEBUG_STATUS>;
#[doc = "Register DEBUG_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_SCL_STATE`"]
pub type RTC_I2C_SCL_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_I2C_SCL_STATE`"]
pub struct RTC_I2C_SCL_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SCL_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_MAIN_STATE`"]
pub type RTC_I2C_MAIN_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_I2C_MAIN_STATE`"]
pub struct RTC_I2C_MAIN_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_MAIN_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_BYTE_TRANS`"]
pub type RTC_I2C_BYTE_TRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_BYTE_TRANS`"]
pub struct RTC_I2C_BYTE_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_BYTE_TRANS_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_SLAVE_ADDR_MATCH`"]
pub type RTC_I2C_SLAVE_ADDR_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_ADDR_MATCH`"]
pub struct RTC_I2C_SLAVE_ADDR_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_ADDR_MATCH_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_BUS_BUSY`"]
pub type RTC_I2C_BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_BUS_BUSY`"]
pub struct RTC_I2C_BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_BUS_BUSY_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_ARB_LOST`"]
pub type RTC_I2C_ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_ARB_LOST`"]
pub struct RTC_I2C_ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ARB_LOST_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TIMED_OUT`"]
pub type RTC_I2C_TIMED_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TIMED_OUT`"]
pub struct RTC_I2C_TIMED_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TIMED_OUT_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_SLAVE_RW`"]
pub type RTC_I2C_SLAVE_RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_RW`"]
pub struct RTC_I2C_SLAVE_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_RW_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_ACK_VAL`"]
pub type RTC_I2C_ACK_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_ACK_VAL`"]
pub struct RTC_I2C_ACK_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_ACK_VAL_W<'a> {
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
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn rtc_i2c_scl_state(&self) -> RTC_I2C_SCL_STATE_R {
        RTC_I2C_SCL_STATE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn rtc_i2c_main_state(&self) -> RTC_I2C_MAIN_STATE_R {
        RTC_I2C_MAIN_STATE_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn rtc_i2c_byte_trans(&self) -> RTC_I2C_BYTE_TRANS_R {
        RTC_I2C_BYTE_TRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn rtc_i2c_slave_addr_match(&self) -> RTC_I2C_SLAVE_ADDR_MATCH_R {
        RTC_I2C_SLAVE_ADDR_MATCH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn rtc_i2c_bus_busy(&self) -> RTC_I2C_BUS_BUSY_R {
        RTC_I2C_BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn rtc_i2c_arb_lost(&self) -> RTC_I2C_ARB_LOST_R {
        RTC_I2C_ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn rtc_i2c_timed_out(&self) -> RTC_I2C_TIMED_OUT_R {
        RTC_I2C_TIMED_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn rtc_i2c_slave_rw(&self) -> RTC_I2C_SLAVE_RW_R {
        RTC_I2C_SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn rtc_i2c_ack_val(&self) -> RTC_I2C_ACK_VAL_R {
        RTC_I2C_ACK_VAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn rtc_i2c_scl_state(&mut self) -> RTC_I2C_SCL_STATE_W {
        RTC_I2C_SCL_STATE_W { w: self }
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn rtc_i2c_main_state(&mut self) -> RTC_I2C_MAIN_STATE_W {
        RTC_I2C_MAIN_STATE_W { w: self }
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn rtc_i2c_byte_trans(&mut self) -> RTC_I2C_BYTE_TRANS_W {
        RTC_I2C_BYTE_TRANS_W { w: self }
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn rtc_i2c_slave_addr_match(&mut self) -> RTC_I2C_SLAVE_ADDR_MATCH_W {
        RTC_I2C_SLAVE_ADDR_MATCH_W { w: self }
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn rtc_i2c_bus_busy(&mut self) -> RTC_I2C_BUS_BUSY_W {
        RTC_I2C_BUS_BUSY_W { w: self }
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn rtc_i2c_arb_lost(&mut self) -> RTC_I2C_ARB_LOST_W {
        RTC_I2C_ARB_LOST_W { w: self }
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn rtc_i2c_timed_out(&mut self) -> RTC_I2C_TIMED_OUT_W {
        RTC_I2C_TIMED_OUT_W { w: self }
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn rtc_i2c_slave_rw(&mut self) -> RTC_I2C_SLAVE_RW_W {
        RTC_I2C_SLAVE_RW_W { w: self }
    }
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn rtc_i2c_ack_val(&mut self) -> RTC_I2C_ACK_VAL_W {
        RTC_I2C_ACK_VAL_W { w: self }
    }
}
