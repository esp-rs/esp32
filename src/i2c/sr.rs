#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCL_STATE_LAST`"]
pub type SCL_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCL_STATE_LAST`"]
pub struct SCL_STATE_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_STATE_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `SCL_MAIN_STATE_LAST`"]
pub type SCL_MAIN_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCL_MAIN_STATE_LAST`"]
pub struct SCL_MAIN_STATE_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_MAIN_STATE_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `TXFIFO_CNT`"]
pub type TXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFIFO_CNT`"]
pub struct TXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RXFIFO_CNT`"]
pub type RXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXFIFO_CNT`"]
pub struct RXFIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `BYTE_TRANS`"]
pub type BYTE_TRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTE_TRANS`"]
pub struct BYTE_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_TRANS_W<'a> {
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
#[doc = "Reader of field `SLAVE_ADDRESSED`"]
pub type SLAVE_ADDRESSED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_ADDRESSED`"]
pub struct SLAVE_ADDRESSED_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDRESSED_W<'a> {
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
#[doc = "Reader of field `BUS_BUSY`"]
pub type BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_BUSY`"]
pub struct BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_BUSY_W<'a> {
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
#[doc = "Reader of field `ARB_LOST`"]
pub type ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARB_LOST`"]
pub struct ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_LOST_W<'a> {
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
#[doc = "Reader of field `TIME_OUT`"]
pub type TIME_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIME_OUT`"]
pub struct TIME_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_W<'a> {
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
#[doc = "Reader of field `SLAVE_RW`"]
pub type SLAVE_RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_RW`"]
pub struct SLAVE_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_RW_W<'a> {
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
#[doc = "Reader of field `ACK_REC`"]
pub type ACK_REC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK_REC`"]
pub struct ACK_REC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_REC_W<'a> {
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
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:23 - This register stores the amount of received data in ram."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - This register represent the amount of data need to send."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This register changes to high level when one byte is transferred."]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - when I2C lost control of SDA line this register changes to high level."]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - when in slave mode 1: master read slave 0: master write slave."]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This register stores the value of ACK bit."]
    #[inline(always)]
    pub fn ack_rec(&self) -> ACK_REC_R {
        ACK_REC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - This register stores the value of state machine to produce SCL. 3'h0: SCL_IDLE 3'h1:SCL_START 3'h2:SCL_LOW_EDGE 3'h3: SCL_LOW 3'h4:SCL_HIGH_EDGE 3'h5:SCL_HIGH 3'h6:SCL_STOP"]
    #[inline(always)]
    pub fn scl_state_last(&mut self) -> SCL_STATE_LAST_W {
        SCL_STATE_LAST_W { w: self }
    }
    #[doc = "Bits 24:26 - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
    #[inline(always)]
    pub fn scl_main_state_last(&mut self) -> SCL_MAIN_STATE_LAST_W {
        SCL_MAIN_STATE_LAST_W { w: self }
    }
    #[doc = "Bits 18:23 - This register stores the amount of received data in ram."]
    #[inline(always)]
    pub fn txfifo_cnt(&mut self) -> TXFIFO_CNT_W {
        TXFIFO_CNT_W { w: self }
    }
    #[doc = "Bits 8:13 - This register represent the amount of data need to send."]
    #[inline(always)]
    pub fn rxfifo_cnt(&mut self) -> RXFIFO_CNT_W {
        RXFIFO_CNT_W { w: self }
    }
    #[doc = "Bit 6 - This register changes to high level when one byte is transferred."]
    #[inline(always)]
    pub fn byte_trans(&mut self) -> BYTE_TRANS_W {
        BYTE_TRANS_W { w: self }
    }
    #[doc = "Bit 5 - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
    #[inline(always)]
    pub fn slave_addressed(&mut self) -> SLAVE_ADDRESSED_W {
        SLAVE_ADDRESSED_W { w: self }
    }
    #[doc = "Bit 4 - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
    #[inline(always)]
    pub fn bus_busy(&mut self) -> BUS_BUSY_W {
        BUS_BUSY_W { w: self }
    }
    #[doc = "Bit 3 - when I2C lost control of SDA line this register changes to high level."]
    #[inline(always)]
    pub fn arb_lost(&mut self) -> ARB_LOST_W {
        ARB_LOST_W { w: self }
    }
    #[doc = "Bit 2 - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W {
        TIME_OUT_W { w: self }
    }
    #[doc = "Bit 1 - when in slave mode 1: master read slave 0: master write slave."]
    #[inline(always)]
    pub fn slave_rw(&mut self) -> SLAVE_RW_W {
        SLAVE_RW_W { w: self }
    }
    #[doc = "Bit 0 - This register stores the value of ACK bit."]
    #[inline(always)]
    pub fn ack_rec(&mut self) -> ACK_REC_W {
        ACK_REC_W { w: self }
    }
}
