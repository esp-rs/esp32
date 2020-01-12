#[doc = "Reader of register INT_RAW"]
pub type R = crate::R<u32, super::INT_RAW>;
#[doc = "Writer for register INT_RAW"]
pub type W = crate::W<u32, super::INT_RAW>;
#[doc = "Register INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_SEND_EMPTY_INT_RAW`"]
pub type TX_SEND_EMPTY_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SEND_EMPTY_INT_RAW`"]
pub struct TX_SEND_EMPTY_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SEND_EMPTY_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RX_REC_FULL_INT_RAW`"]
pub type RX_REC_FULL_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_REC_FULL_INT_RAW`"]
pub struct RX_REC_FULL_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_REC_FULL_INT_RAW_W<'a> {
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
#[doc = "Reader of field `ACK_ERR_INT_RAW`"]
pub type ACK_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK_ERR_INT_RAW`"]
pub struct ACK_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_ERR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TRANS_START_INT_RAW`"]
pub type TRANS_START_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_START_INT_RAW`"]
pub struct TRANS_START_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_START_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TIME_OUT_INT_RAW`"]
pub type TIME_OUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIME_OUT_INT_RAW`"]
pub struct TIME_OUT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_OUT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TRANS_COMPLETE_INT_RAW`"]
pub type TRANS_COMPLETE_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_COMPLETE_INT_RAW`"]
pub struct TRANS_COMPLETE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMPLETE_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MASTER_TRAN_COMP_INT_RAW`"]
pub type MASTER_TRAN_COMP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER_TRAN_COMP_INT_RAW`"]
pub struct MASTER_TRAN_COMP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_TRAN_COMP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `ARBITRATION_LOST_INT_RAW`"]
pub type ARBITRATION_LOST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBITRATION_LOST_INT_RAW`"]
pub struct ARBITRATION_LOST_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBITRATION_LOST_INT_RAW_W<'a> {
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
#[doc = "Reader of field `SLAVE_TRAN_COMP_INT_RAW`"]
pub type SLAVE_TRAN_COMP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_TRAN_COMP_INT_RAW`"]
pub struct SLAVE_TRAN_COMP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TRAN_COMP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `END_DETECT_INT_RAW`"]
pub type END_DETECT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_DETECT_INT_RAW`"]
pub struct END_DETECT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DETECT_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RXFIFO_OVF_INT_RAW`"]
pub type RXFIFO_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_OVF_INT_RAW`"]
pub struct RXFIFO_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `TXFIFO_EMPTY_INT_RAW`"]
pub type TXFIFO_EMPTY_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_EMPTY_INT_RAW`"]
pub struct TXFIFO_EMPTY_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_INT_RAW_W<'a> {
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
#[doc = "Reader of field `RXFIFO_FULL_INT_RAW`"]
pub type RXFIFO_FULL_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFO_FULL_INT_RAW`"]
pub struct RXFIFO_FULL_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_INT_RAW_W<'a> {
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
    #[doc = "Bit 12 - The raw interrupt status bit for tx_send_empty_int interrupt.when I2C sends more data than nonfifo_tx_thres it will produce tx_send_empty_int interrupt.."]
    #[inline(always)]
    pub fn tx_send_empty_int_raw(&self) -> TX_SEND_EMPTY_INT_RAW_R {
        TX_SEND_EMPTY_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for rx_rec_full_int interrupt. when I2C receives more data than nonfifo_rx_thres it will produce rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_raw(&self) -> RX_REC_FULL_INT_RAW_R {
        RX_REC_FULL_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for ack_err_int interrupt. when I2C receives a wrong ACK bit it will produce ack_err_int interrupt.."]
    #[inline(always)]
    pub fn ack_err_int_raw(&self) -> ACK_ERR_INT_RAW_R {
        ACK_ERR_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for trans_start_int interrupt. when I2C sends the START bit it will produce trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_raw(&self) -> TRANS_START_INT_RAW_R {
        TRANS_START_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for time_out_int interrupt. when I2C takes a lot of time to receive a data it will produce time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for trans_complete_int interrupt. when I2C Master finished STOP command it will produce trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for master_tra_comp_int interrupt. when I2C Master sends or receives a byte it will produce master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_raw(&self) -> MASTER_TRAN_COMP_INT_RAW_R {
        MASTER_TRAN_COMP_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for arbitration_lost_int interrupt.when I2C lost the usage right of I2C BUS it will produce arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for slave_tran_comp_int interrupt. when I2C Slave detectsthe STOP bit it will produce slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_raw(&self) -> SLAVE_TRAN_COMP_INT_RAW_R {
        SLAVE_TRAN_COMP_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for end_detect_int interrupt. when I2C deals with the END command it will produce end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_raw(&self) -> END_DETECT_INT_RAW_R {
        END_DETECT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for receiving data overflow when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for txfifo empty when use apb fifo access."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The raw interrupt status bit for rxfifo full when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - The raw interrupt status bit for tx_send_empty_int interrupt.when I2C sends more data than nonfifo_tx_thres it will produce tx_send_empty_int interrupt.."]
    #[inline(always)]
    pub fn tx_send_empty_int_raw(&mut self) -> TX_SEND_EMPTY_INT_RAW_W {
        TX_SEND_EMPTY_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11 - The raw interrupt status bit for rx_rec_full_int interrupt. when I2C receives more data than nonfifo_rx_thres it will produce rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_raw(&mut self) -> RX_REC_FULL_INT_RAW_W {
        RX_REC_FULL_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10 - The raw interrupt status bit for ack_err_int interrupt. when I2C receives a wrong ACK bit it will produce ack_err_int interrupt.."]
    #[inline(always)]
    pub fn ack_err_int_raw(&mut self) -> ACK_ERR_INT_RAW_W {
        ACK_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9 - The raw interrupt status bit for trans_start_int interrupt. when I2C sends the START bit it will produce trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_raw(&mut self) -> TRANS_START_INT_RAW_W {
        TRANS_START_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8 - The raw interrupt status bit for time_out_int interrupt. when I2C takes a lot of time to receive a data it will produce time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_raw(&mut self) -> TIME_OUT_INT_RAW_W {
        TIME_OUT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7 - The raw interrupt status bit for trans_complete_int interrupt. when I2C Master finished STOP command it will produce trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_raw(&mut self) -> TRANS_COMPLETE_INT_RAW_W {
        TRANS_COMPLETE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - The raw interrupt status bit for master_tra_comp_int interrupt. when I2C Master sends or receives a byte it will produce master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_raw(&mut self) -> MASTER_TRAN_COMP_INT_RAW_W {
        MASTER_TRAN_COMP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - The raw interrupt status bit for arbitration_lost_int interrupt.when I2C lost the usage right of I2C BUS it will produce arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&mut self) -> ARBITRATION_LOST_INT_RAW_W {
        ARBITRATION_LOST_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - The raw interrupt status bit for slave_tran_comp_int interrupt. when I2C Slave detectsthe STOP bit it will produce slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_raw(&mut self) -> SLAVE_TRAN_COMP_INT_RAW_W {
        SLAVE_TRAN_COMP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - The raw interrupt status bit for end_detect_int interrupt. when I2C deals with the END command it will produce end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_raw(&mut self) -> END_DETECT_INT_RAW_W {
        END_DETECT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - The raw interrupt status bit for receiving data overflow when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&mut self) -> RXFIFO_OVF_INT_RAW_W {
        RXFIFO_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - The raw interrupt status bit for txfifo empty when use apb fifo access."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&mut self) -> TXFIFO_EMPTY_INT_RAW_W {
        TXFIFO_EMPTY_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - The raw interrupt status bit for rxfifo full when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&mut self) -> RXFIFO_FULL_INT_RAW_W {
        RXFIFO_FULL_INT_RAW_W { w: self }
    }
}
