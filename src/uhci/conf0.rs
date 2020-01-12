#[doc = "Reader of register CONF0"]
pub type R = crate::R<u32, super::CONF0>;
#[doc = "Writer for register CONF0"]
pub type W = crate::W<u32, super::CONF0>;
#[doc = "Register CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RX_BRK_EOF_EN`"]
pub type UART_RX_BRK_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_RX_BRK_EOF_EN`"]
pub struct UART_RX_BRK_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_BRK_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ENCODE_CRC_EN`"]
pub type ENCODE_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCODE_CRC_EN`"]
pub struct ENCODE_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCODE_CRC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LEN_EOF_EN`"]
pub type LEN_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEN_EOF_EN`"]
pub struct LEN_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `UART_IDLE_EOF_EN`"]
pub type UART_IDLE_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_IDLE_EOF_EN`"]
pub struct UART_IDLE_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IDLE_EOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `CRC_REC_EN`"]
pub type CRC_REC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_REC_EN`"]
pub struct CRC_REC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_REC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `HEAD_EN`"]
pub type HEAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HEAD_EN`"]
pub struct HEAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HEAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SEPER_EN`"]
pub type SEPER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEPER_EN`"]
pub struct SEPER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_TRANS_EN`"]
pub type MEM_TRANS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_TRANS_EN`"]
pub struct MEM_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `OUT_DATA_BURST_EN`"]
pub type OUT_DATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_DATA_BURST_EN`"]
pub struct OUT_DATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `INDSCR_BURST_EN`"]
pub type INDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDSCR_BURST_EN`"]
pub struct INDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `OUTDSCR_BURST_EN`"]
pub type OUTDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTDSCR_BURST_EN`"]
pub struct OUTDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDSCR_BURST_EN_W<'a> {
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
#[doc = "Reader of field `UART2_CE`"]
pub type UART2_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART2_CE`"]
pub struct UART2_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_CE_W<'a> {
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
#[doc = "Reader of field `UART1_CE`"]
pub type UART1_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_CE`"]
pub struct UART1_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CE_W<'a> {
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
#[doc = "Reader of field `UART0_CE`"]
pub type UART0_CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_CE`"]
pub struct UART0_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_CE_W<'a> {
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
#[doc = "Reader of field `OUT_EOF_MODE`"]
pub type OUT_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_EOF_MODE`"]
pub struct OUT_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_MODE_W<'a> {
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
#[doc = "Reader of field `OUT_NO_RESTART_CLR`"]
pub type OUT_NO_RESTART_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_NO_RESTART_CLR`"]
pub struct OUT_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_NO_RESTART_CLR_W<'a> {
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
#[doc = "Reader of field `OUT_AUTO_WRBACK`"]
pub type OUT_AUTO_WRBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_AUTO_WRBACK`"]
pub struct OUT_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_AUTO_WRBACK_W<'a> {
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
#[doc = "Reader of field `OUT_LOOP_TEST`"]
pub type OUT_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_LOOP_TEST`"]
pub struct OUT_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_LOOP_TEST_W<'a> {
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
#[doc = "Reader of field `IN_LOOP_TEST`"]
pub type IN_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_LOOP_TEST`"]
pub struct IN_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_LOOP_TEST_W<'a> {
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
#[doc = "Reader of field `AHBM_RST`"]
pub type AHBM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBM_RST`"]
pub struct AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_RST_W<'a> {
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
#[doc = "Reader of field `AHBM_FIFO_RST`"]
pub type AHBM_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBM_FIFO_RST`"]
pub struct AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_FIFO_RST_W<'a> {
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
#[doc = "Reader of field `OUT_RST`"]
pub type OUT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_RST`"]
pub struct OUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_RST_W<'a> {
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
#[doc = "Reader of field `IN_RST`"]
pub type IN_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_RST`"]
pub struct IN_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_RST_W<'a> {
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
    #[doc = "Bit 23 - Set this bit to enable to use brk char as the end of a data frame."]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&self) -> UART_RX_BRK_EOF_EN_R {
        UART_RX_BRK_EOF_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable clock-gating for read or write registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
    #[inline(always)]
    pub fn encode_crc_en(&self) -> ENCODE_CRC_EN_R {
        ENCODE_CRC_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
    #[inline(always)]
    pub fn len_eof_en(&self) -> LEN_EOF_EN_R {
        LEN_EOF_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> UART_IDLE_EOF_EN_R {
        UART_IDLE_EOF_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
    #[inline(always)]
    pub fn crc_rec_en(&self) -> CRC_REC_EN_R {
        CRC_REC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable to use head packet before the data frame."]
    #[inline(always)]
    pub fn head_en(&self) -> HEAD_EN_R {
        HEAD_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set this bit to use special char to separate the data frame."]
    #[inline(always)]
    pub fn seper_en(&self) -> SEPER_EN_R {
        SEPER_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable DMA burst MODE"]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable DMA out links to use burst mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable DMA in links to use burst mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set this bit to use UART2 to transmit or receive data."]
    #[inline(always)]
    pub fn uart2_ce(&self) -> UART2_CE_R {
        UART2_CE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set this bit to use UART1 to transmit or receive data."]
    #[inline(always)]
    pub fn uart1_ce(&self) -> UART1_CE_R {
        UART1_CE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set this bit to use UART to transmit or receive data."]
    #[inline(always)]
    pub fn uart0_ce(&self) -> UART0_CE_R {
        UART0_CE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - don't use"]
    #[inline(always)]
    pub fn out_no_restart_clr(&self) -> OUT_NO_RESTART_CLR_R {
        OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - when in link's length is 0 go on to use the next in link automatically."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable loop test for out links."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable loop test for in links."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset dma ahb interface."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset dma ahb fifo."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out link operations."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to reset in link operations."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Set this bit to enable to use brk char as the end of a data frame."]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W {
        UART_RX_BRK_EOF_EN_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to enable clock-gating for read or write registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
    #[inline(always)]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W {
        ENCODE_CRC_EN_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
    #[inline(always)]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W {
        LEN_EOF_EN_W { w: self }
    }
    #[doc = "Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W {
        UART_IDLE_EOF_EN_W { w: self }
    }
    #[doc = "Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
    #[inline(always)]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W {
        CRC_REC_EN_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to enable to use head packet before the data frame."]
    #[inline(always)]
    pub fn head_en(&mut self) -> HEAD_EN_W {
        HEAD_EN_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to use special char to separate the data frame."]
    #[inline(always)]
    pub fn seper_en(&mut self) -> SEPER_EN_W {
        SEPER_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W {
        MEM_TRANS_EN_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to enable DMA burst MODE"]
    #[inline(always)]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W {
        OUT_DATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to enable DMA out links to use burst mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W {
        INDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to enable DMA in links to use burst mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W {
        OUTDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to use UART2 to transmit or receive data."]
    #[inline(always)]
    pub fn uart2_ce(&mut self) -> UART2_CE_W {
        UART2_CE_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to use UART1 to transmit or receive data."]
    #[inline(always)]
    pub fn uart1_ce(&mut self) -> UART1_CE_W {
        UART1_CE_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to use UART to transmit or receive data."]
    #[inline(always)]
    pub fn uart0_ce(&mut self) -> UART0_CE_W {
        UART0_CE_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W {
        OUT_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 7 - don't use"]
    #[inline(always)]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W {
        OUT_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 6 - when in link's length is 0 go on to use the next in link automatically."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W {
        OUT_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to enable loop test for out links."]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W {
        OUT_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to enable loop test for in links."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W {
        IN_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to reset dma ahb interface."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W {
        AHBM_RST_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset dma ahb fifo."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W {
        AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to reset out link operations."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W {
        OUT_RST_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to reset in link operations."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W {
        IN_RST_W { w: self }
    }
}
