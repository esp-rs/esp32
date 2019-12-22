#[doc = "Reader of register I2C_SR_REG"]
pub type R = crate::R<u32, super::I2C_SR_REG>;
#[doc = "Writer for register I2C_SR_REG"]
pub type W = crate::W<u32, super::I2C_SR_REG>;
#[doc = "Register I2C_SR_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_SR_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SCL_STATE_LAST`"]
pub type I2C_SCL_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SCL_STATE_LAST`"]
pub struct I2C_SCL_STATE_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_STATE_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCL_MAIN_STATE_LAST`"]
pub type I2C_SCL_MAIN_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_SCL_MAIN_STATE_LAST`"]
pub struct I2C_SCL_MAIN_STATE_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_MAIN_STATE_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2C_TXFIFO_CNT`"]
pub type I2C_TXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_TXFIFO_CNT`"]
pub struct I2C_TXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `I2C_RXFIFO_CNT`"]
pub type I2C_RXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_RXFIFO_CNT`"]
pub struct I2C_RXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C_BYTE_TRANS`"]
pub type I2C_BYTE_TRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_BYTE_TRANS`"]
pub struct I2C_BYTE_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_BYTE_TRANS_W<'a> {
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
#[doc = "Reader of field `I2C_SLAVE_ADDRESSED`"]
pub type I2C_SLAVE_ADDRESSED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLAVE_ADDRESSED`"]
pub struct I2C_SLAVE_ADDRESSED_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDRESSED_W<'a> {
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
#[doc = "Reader of field `I2C_BUS_BUSY`"]
pub type I2C_BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_BUS_BUSY`"]
pub struct I2C_BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_BUS_BUSY_W<'a> {
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
#[doc = "Reader of field `I2C_ARB_LOST`"]
pub type I2C_ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ARB_LOST`"]
pub struct I2C_ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARB_LOST_W<'a> {
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
#[doc = "Reader of field `I2C_TIME_OUT`"]
pub type I2C_TIME_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TIME_OUT`"]
pub struct I2C_TIME_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TIME_OUT_W<'a> {
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
#[doc = "Reader of field `I2C_SLAVE_RW`"]
pub type I2C_SLAVE_RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLAVE_RW`"]
pub struct I2C_SLAVE_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_RW_W<'a> {
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
#[doc = "Reader of field `I2C_ACK_REC`"]
pub type I2C_ACK_REC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ACK_REC`"]
pub struct I2C_ACK_REC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ACK_REC_W<'a> {
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
    #[doc = "Bits 28:30 - This register stores the value of state machine to produce SCL. 3'h0: SCL_IDLE 3'h1:SCL_START 3'h2:SCL_LOW_EDGE 3'h3: SCL_LOW 3'h4:SCL_HIGH_EDGE 3'h5:SCL_HIGH 3'h6:SCL_STOP"]
    #[inline(always)]
    pub fn i2c_scl_state_last(&self) -> I2C_SCL_STATE_LAST_R {
        I2C_SCL_STATE_LAST_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
    #[inline(always)]
    pub fn i2c_scl_main_state_last(&self) -> I2C_SCL_MAIN_STATE_LAST_R {
        I2C_SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:23 - This register stores the amount of received data in ram."]
    #[inline(always)]
    pub fn i2c_txfifo_cnt(&self) -> I2C_TXFIFO_CNT_R {
        I2C_TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - This register represent the amount of data need to send."]
    #[inline(always)]
    pub fn i2c_rxfifo_cnt(&self) -> I2C_RXFIFO_CNT_R {
        I2C_RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This register changes to high level when one byte is transferred."]
    #[inline(always)]
    pub fn i2c_byte_trans(&self) -> I2C_BYTE_TRANS_R {
        I2C_BYTE_TRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
    #[inline(always)]
    pub fn i2c_slave_addressed(&self) -> I2C_SLAVE_ADDRESSED_R {
        I2C_SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
    #[inline(always)]
    pub fn i2c_bus_busy(&self) -> I2C_BUS_BUSY_R {
        I2C_BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - when I2C lost control of SDA line this register changes to high level."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
    #[inline(always)]
    pub fn i2c_time_out(&self) -> I2C_TIME_OUT_R {
        I2C_TIME_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - when in slave mode 1: master read slave 0: master write slave."]
    #[inline(always)]
    pub fn i2c_slave_rw(&self) -> I2C_SLAVE_RW_R {
        I2C_SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This register stores the value of ACK bit."]
    #[inline(always)]
    pub fn i2c_ack_rec(&self) -> I2C_ACK_REC_R {
        I2C_ACK_REC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - This register stores the value of state machine to produce SCL. 3'h0: SCL_IDLE 3'h1:SCL_START 3'h2:SCL_LOW_EDGE 3'h3: SCL_LOW 3'h4:SCL_HIGH_EDGE 3'h5:SCL_HIGH 3'h6:SCL_STOP"]
    #[inline(always)]
    pub fn i2c_scl_state_last(&mut self) -> I2C_SCL_STATE_LAST_W {
        I2C_SCL_STATE_LAST_W { w: self }
    }
    #[doc = "Bits 24:26 - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
    #[inline(always)]
    pub fn i2c_scl_main_state_last(&mut self) -> I2C_SCL_MAIN_STATE_LAST_W {
        I2C_SCL_MAIN_STATE_LAST_W { w: self }
    }
    #[doc = "Bits 18:23 - This register stores the amount of received data in ram."]
    #[inline(always)]
    pub fn i2c_txfifo_cnt(&mut self) -> I2C_TXFIFO_CNT_W {
        I2C_TXFIFO_CNT_W { w: self }
    }
    #[doc = "Bits 8:13 - This register represent the amount of data need to send."]
    #[inline(always)]
    pub fn i2c_rxfifo_cnt(&mut self) -> I2C_RXFIFO_CNT_W {
        I2C_RXFIFO_CNT_W { w: self }
    }
    #[doc = "Bit 6 - This register changes to high level when one byte is transferred."]
    #[inline(always)]
    pub fn i2c_byte_trans(&mut self) -> I2C_BYTE_TRANS_W {
        I2C_BYTE_TRANS_W { w: self }
    }
    #[doc = "Bit 5 - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
    #[inline(always)]
    pub fn i2c_slave_addressed(&mut self) -> I2C_SLAVE_ADDRESSED_W {
        I2C_SLAVE_ADDRESSED_W { w: self }
    }
    #[doc = "Bit 4 - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
    #[inline(always)]
    pub fn i2c_bus_busy(&mut self) -> I2C_BUS_BUSY_W {
        I2C_BUS_BUSY_W { w: self }
    }
    #[doc = "Bit 3 - when I2C lost control of SDA line this register changes to high level."]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W {
        I2C_ARB_LOST_W { w: self }
    }
    #[doc = "Bit 2 - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
    #[inline(always)]
    pub fn i2c_time_out(&mut self) -> I2C_TIME_OUT_W {
        I2C_TIME_OUT_W { w: self }
    }
    #[doc = "Bit 1 - when in slave mode 1: master read slave 0: master write slave."]
    #[inline(always)]
    pub fn i2c_slave_rw(&mut self) -> I2C_SLAVE_RW_W {
        I2C_SLAVE_RW_W { w: self }
    }
    #[doc = "Bit 0 - This register stores the value of ACK bit."]
    #[inline(always)]
    pub fn i2c_ack_rec(&mut self) -> I2C_ACK_REC_W {
        I2C_ACK_REC_W { w: self }
    }
}
