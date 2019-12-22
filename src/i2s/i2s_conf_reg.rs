#[doc = "Reader of register I2S_CONF_REG"]
pub type R = crate::R<u32, super::I2S_CONF_REG>;
#[doc = "Writer for register I2S_CONF_REG"]
pub type W = crate::W<u32, super::I2S_CONF_REG>;
#[doc = "Register I2S_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_SIG_LOOPBACK`"]
pub type I2S_SIG_LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_SIG_LOOPBACK`"]
pub struct I2S_SIG_LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SIG_LOOPBACK_W<'a> {
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
#[doc = "Reader of field `I2S_RX_MSB_RIGHT`"]
pub type I2S_RX_MSB_RIGHT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_MSB_RIGHT`"]
pub struct I2S_RX_MSB_RIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_MSB_RIGHT_W<'a> {
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
#[doc = "Reader of field `I2S_TX_MSB_RIGHT`"]
pub type I2S_TX_MSB_RIGHT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_MSB_RIGHT`"]
pub struct I2S_TX_MSB_RIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_MSB_RIGHT_W<'a> {
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
#[doc = "Reader of field `I2S_RX_MONO`"]
pub type I2S_RX_MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_MONO`"]
pub struct I2S_RX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_MONO_W<'a> {
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
#[doc = "Reader of field `I2S_TX_MONO`"]
pub type I2S_TX_MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_MONO`"]
pub struct I2S_TX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_MONO_W<'a> {
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
#[doc = "Reader of field `I2S_RX_SHORT_SYNC`"]
pub type I2S_RX_SHORT_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_SHORT_SYNC`"]
pub struct I2S_RX_SHORT_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_SHORT_SYNC_W<'a> {
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
#[doc = "Reader of field `I2S_TX_SHORT_SYNC`"]
pub type I2S_TX_SHORT_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_SHORT_SYNC`"]
pub struct I2S_TX_SHORT_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_SHORT_SYNC_W<'a> {
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
#[doc = "Reader of field `I2S_RX_MSB_SHIFT`"]
pub type I2S_RX_MSB_SHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_MSB_SHIFT`"]
pub struct I2S_RX_MSB_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_MSB_SHIFT_W<'a> {
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
#[doc = "Reader of field `I2S_TX_MSB_SHIFT`"]
pub type I2S_TX_MSB_SHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_MSB_SHIFT`"]
pub struct I2S_TX_MSB_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_MSB_SHIFT_W<'a> {
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
#[doc = "Reader of field `I2S_RX_RIGHT_FIRST`"]
pub type I2S_RX_RIGHT_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_RIGHT_FIRST`"]
pub struct I2S_RX_RIGHT_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_RIGHT_FIRST_W<'a> {
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
#[doc = "Reader of field `I2S_TX_RIGHT_FIRST`"]
pub type I2S_TX_RIGHT_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_RIGHT_FIRST`"]
pub struct I2S_TX_RIGHT_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_RIGHT_FIRST_W<'a> {
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
#[doc = "Reader of field `I2S_RX_SLAVE_MOD`"]
pub type I2S_RX_SLAVE_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_SLAVE_MOD`"]
pub struct I2S_RX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_SLAVE_MOD_W<'a> {
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
#[doc = "Reader of field `I2S_TX_SLAVE_MOD`"]
pub type I2S_TX_SLAVE_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_SLAVE_MOD`"]
pub struct I2S_TX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_SLAVE_MOD_W<'a> {
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
#[doc = "Reader of field `I2S_RX_START`"]
pub type I2S_RX_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_START`"]
pub struct I2S_RX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_START_W<'a> {
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
#[doc = "Reader of field `I2S_TX_START`"]
pub type I2S_TX_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_START`"]
pub struct I2S_TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_START_W<'a> {
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
#[doc = "Reader of field `I2S_RX_FIFO_RESET`"]
pub type I2S_RX_FIFO_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_FIFO_RESET`"]
pub struct I2S_RX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_FIFO_RESET_W<'a> {
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
#[doc = "Reader of field `I2S_TX_FIFO_RESET`"]
pub type I2S_TX_FIFO_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_FIFO_RESET`"]
pub struct I2S_TX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_FIFO_RESET_W<'a> {
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
#[doc = "Reader of field `I2S_RX_RESET`"]
pub type I2S_RX_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RX_RESET`"]
pub struct I2S_RX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_RESET_W<'a> {
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
#[doc = "Reader of field `I2S_TX_RESET`"]
pub type I2S_TX_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_RESET`"]
pub struct I2S_TX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_RESET_W<'a> {
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
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2s_sig_loopback(&self) -> I2S_SIG_LOOPBACK_R {
        I2S_SIG_LOOPBACK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2s_rx_msb_right(&self) -> I2S_RX_MSB_RIGHT_R {
        I2S_RX_MSB_RIGHT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_tx_msb_right(&self) -> I2S_TX_MSB_RIGHT_R {
        I2S_TX_MSB_RIGHT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2s_rx_mono(&self) -> I2S_RX_MONO_R {
        I2S_RX_MONO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2s_tx_mono(&self) -> I2S_TX_MONO_R {
        I2S_TX_MONO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2s_rx_short_sync(&self) -> I2S_RX_SHORT_SYNC_R {
        I2S_RX_SHORT_SYNC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_tx_short_sync(&self) -> I2S_TX_SHORT_SYNC_R {
        I2S_TX_SHORT_SYNC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_rx_msb_shift(&self) -> I2S_RX_MSB_SHIFT_R {
        I2S_RX_MSB_SHIFT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s_tx_msb_shift(&self) -> I2S_TX_MSB_SHIFT_R {
        I2S_TX_MSB_SHIFT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_rx_right_first(&self) -> I2S_RX_RIGHT_FIRST_R {
        I2S_RX_RIGHT_FIRST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_tx_right_first(&self) -> I2S_TX_RIGHT_FIRST_R {
        I2S_TX_RIGHT_FIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_rx_slave_mod(&self) -> I2S_RX_SLAVE_MOD_R {
        I2S_RX_SLAVE_MOD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_tx_slave_mod(&self) -> I2S_TX_SLAVE_MOD_R {
        I2S_TX_SLAVE_MOD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_rx_start(&self) -> I2S_RX_START_R {
        I2S_RX_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_tx_start(&self) -> I2S_TX_START_R {
        I2S_TX_START_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_rx_fifo_reset(&self) -> I2S_RX_FIFO_RESET_R {
        I2S_RX_FIFO_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_tx_fifo_reset(&self) -> I2S_TX_FIFO_RESET_R {
        I2S_TX_FIFO_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_rx_reset(&self) -> I2S_RX_RESET_R {
        I2S_RX_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_tx_reset(&self) -> I2S_TX_RESET_R {
        I2S_TX_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2s_sig_loopback(&mut self) -> I2S_SIG_LOOPBACK_W {
        I2S_SIG_LOOPBACK_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2s_rx_msb_right(&mut self) -> I2S_RX_MSB_RIGHT_W {
        I2S_RX_MSB_RIGHT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_tx_msb_right(&mut self) -> I2S_TX_MSB_RIGHT_W {
        I2S_TX_MSB_RIGHT_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2s_rx_mono(&mut self) -> I2S_RX_MONO_W {
        I2S_RX_MONO_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2s_tx_mono(&mut self) -> I2S_TX_MONO_W {
        I2S_TX_MONO_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2s_rx_short_sync(&mut self) -> I2S_RX_SHORT_SYNC_W {
        I2S_RX_SHORT_SYNC_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_tx_short_sync(&mut self) -> I2S_TX_SHORT_SYNC_W {
        I2S_TX_SHORT_SYNC_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_rx_msb_shift(&mut self) -> I2S_RX_MSB_SHIFT_W {
        I2S_RX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s_tx_msb_shift(&mut self) -> I2S_TX_MSB_SHIFT_W {
        I2S_TX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_rx_right_first(&mut self) -> I2S_RX_RIGHT_FIRST_W {
        I2S_RX_RIGHT_FIRST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_tx_right_first(&mut self) -> I2S_TX_RIGHT_FIRST_W {
        I2S_TX_RIGHT_FIRST_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_rx_slave_mod(&mut self) -> I2S_RX_SLAVE_MOD_W {
        I2S_RX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_tx_slave_mod(&mut self) -> I2S_TX_SLAVE_MOD_W {
        I2S_TX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_rx_start(&mut self) -> I2S_RX_START_W {
        I2S_RX_START_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_tx_start(&mut self) -> I2S_TX_START_W {
        I2S_TX_START_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_rx_fifo_reset(&mut self) -> I2S_RX_FIFO_RESET_W {
        I2S_RX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_tx_fifo_reset(&mut self) -> I2S_TX_FIFO_RESET_W {
        I2S_TX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_rx_reset(&mut self) -> I2S_RX_RESET_W {
        I2S_RX_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_tx_reset(&mut self) -> I2S_TX_RESET_W {
        I2S_TX_RESET_W { w: self }
    }
}
