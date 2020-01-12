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
#[doc = "Reader of field `SCL_STATE`"]
pub type SCL_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCL_STATE`"]
pub struct SCL_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `MAIN_STATE`"]
pub type MAIN_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAIN_STATE`"]
pub struct MAIN_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
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
#[doc = "Reader of field `SLAVE_ADDR_MATCH`"]
pub type SLAVE_ADDR_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_ADDR_MATCH`"]
pub struct SLAVE_ADDR_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_MATCH_W<'a> {
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
#[doc = "Reader of field `TIMED_OUT`"]
pub type TIMED_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMED_OUT`"]
pub struct TIMED_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMED_OUT_W<'a> {
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
#[doc = "Reader of field `ACK_VAL`"]
pub type ACK_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK_VAL`"]
pub struct ACK_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_VAL_W<'a> {
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
    pub fn scl_state(&self) -> SCL_STATE_R {
        SCL_STATE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn slave_addr_match(&self) -> SLAVE_ADDR_MATCH_R {
        SLAVE_ADDR_MATCH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn timed_out(&self) -> TIMED_OUT_R {
        TIMED_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn ack_val(&self) -> ACK_VAL_R {
        ACK_VAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn scl_state(&mut self) -> SCL_STATE_W {
        SCL_STATE_W { w: self }
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn main_state(&mut self) -> MAIN_STATE_W {
        MAIN_STATE_W { w: self }
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn byte_trans(&mut self) -> BYTE_TRANS_W {
        BYTE_TRANS_W { w: self }
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn slave_addr_match(&mut self) -> SLAVE_ADDR_MATCH_W {
        SLAVE_ADDR_MATCH_W { w: self }
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn bus_busy(&mut self) -> BUS_BUSY_W {
        BUS_BUSY_W { w: self }
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn arb_lost(&mut self) -> ARB_LOST_W {
        ARB_LOST_W { w: self }
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn timed_out(&mut self) -> TIMED_OUT_W {
        TIMED_OUT_W { w: self }
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn slave_rw(&mut self) -> SLAVE_RW_W {
        SLAVE_RW_W { w: self }
    }
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn ack_val(&mut self) -> ACK_VAL_W {
        ACK_VAL_W { w: self }
    }
}
