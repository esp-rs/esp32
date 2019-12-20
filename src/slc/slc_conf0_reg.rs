#[doc = "Reader of register SLC_CONF0_REG"]
pub type R = crate::R<u32, super::SLC_CONF0_REG>;
#[doc = "Writer for register SLC_CONF0_REG"]
pub type W = crate::W<u32, super::SLC_CONF0_REG>;
#[doc = "Register SLC_CONF0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_CONF0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_TOKEN_SEL`"]
pub type SLC_SLC1_TOKEN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN_SEL`"]
pub struct SLC_SLC1_TOKEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN_SEL_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TOKEN_AUTO_CLR`"]
pub type SLC_SLC1_TOKEN_AUTO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN_AUTO_CLR`"]
pub struct SLC_SLC1_TOKEN_AUTO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN_AUTO_CLR_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TXDATA_BURST_EN`"]
pub type SLC_SLC1_TXDATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TXDATA_BURST_EN`"]
pub struct SLC_SLC1_TXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TXDATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_TXDSCR_BURST_EN`"]
pub type SLC_SLC1_TXDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TXDSCR_BURST_EN`"]
pub struct SLC_SLC1_TXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TXDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_TXLINK_AUTO_RET`"]
pub type SLC_SLC1_TXLINK_AUTO_RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TXLINK_AUTO_RET`"]
pub struct SLC_SLC1_TXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TXLINK_AUTO_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RXLINK_AUTO_RET`"]
pub type SLC_SLC1_RXLINK_AUTO_RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RXLINK_AUTO_RET`"]
pub struct SLC_SLC1_RXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXLINK_AUTO_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RXDATA_BURST_EN`"]
pub type SLC_SLC1_RXDATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RXDATA_BURST_EN`"]
pub struct SLC_SLC1_RXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXDATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RXDSCR_BURST_EN`"]
pub type SLC_SLC1_RXDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RXDSCR_BURST_EN`"]
pub struct SLC_SLC1_RXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RX_NO_RESTART_CLR`"]
pub type SLC_SLC1_RX_NO_RESTART_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_NO_RESTART_CLR`"]
pub struct SLC_SLC1_RX_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_NO_RESTART_CLR_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_AUTO_WRBACK`"]
pub type SLC_SLC1_RX_AUTO_WRBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_AUTO_WRBACK`"]
pub struct SLC_SLC1_RX_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_AUTO_WRBACK_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_LOOP_TEST`"]
pub type SLC_SLC1_RX_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_LOOP_TEST`"]
pub struct SLC_SLC1_RX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_LOOP_TEST_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_LOOP_TEST`"]
pub type SLC_SLC1_TX_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_LOOP_TEST`"]
pub struct SLC_SLC1_TX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_LOOP_TEST_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_WR_RETRY_MASK_EN`"]
pub type SLC_SLC1_WR_RETRY_MASK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_WR_RETRY_MASK_EN`"]
pub struct SLC_SLC1_WR_RETRY_MASK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_WR_RETRY_MASK_EN_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_WR_RETRY_MASK_EN`"]
pub type SLC_SLC0_WR_RETRY_MASK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_WR_RETRY_MASK_EN`"]
pub struct SLC_SLC0_WR_RETRY_MASK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_WR_RETRY_MASK_EN_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RX_RST`"]
pub type SLC_SLC1_RX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_RST`"]
pub struct SLC_SLC1_RX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_RST_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_TX_RST`"]
pub type SLC_SLC1_TX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_RST`"]
pub struct SLC_SLC1_TX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_RST_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TOKEN_SEL`"]
pub type SLC_SLC0_TOKEN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TOKEN_SEL`"]
pub struct SLC_SLC0_TOKEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TOKEN_SEL_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TOKEN_AUTO_CLR`"]
pub type SLC_SLC0_TOKEN_AUTO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TOKEN_AUTO_CLR`"]
pub struct SLC_SLC0_TOKEN_AUTO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TOKEN_AUTO_CLR_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TXDATA_BURST_EN`"]
pub type SLC_SLC0_TXDATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TXDATA_BURST_EN`"]
pub struct SLC_SLC0_TXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TXDATA_BURST_EN_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TXDSCR_BURST_EN`"]
pub type SLC_SLC0_TXDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TXDSCR_BURST_EN`"]
pub struct SLC_SLC0_TXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TXDSCR_BURST_EN_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TXLINK_AUTO_RET`"]
pub type SLC_SLC0_TXLINK_AUTO_RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TXLINK_AUTO_RET`"]
pub struct SLC_SLC0_TXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TXLINK_AUTO_RET_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RXLINK_AUTO_RET`"]
pub type SLC_SLC0_RXLINK_AUTO_RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RXLINK_AUTO_RET`"]
pub struct SLC_SLC0_RXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RXLINK_AUTO_RET_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RXDATA_BURST_EN`"]
pub type SLC_SLC0_RXDATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RXDATA_BURST_EN`"]
pub struct SLC_SLC0_RXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RXDATA_BURST_EN_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RXDSCR_BURST_EN`"]
pub type SLC_SLC0_RXDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RXDSCR_BURST_EN`"]
pub struct SLC_SLC0_RXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RXDSCR_BURST_EN_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_NO_RESTART_CLR`"]
pub type SLC_SLC0_RX_NO_RESTART_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_NO_RESTART_CLR`"]
pub struct SLC_SLC0_RX_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_NO_RESTART_CLR_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_AUTO_WRBACK`"]
pub type SLC_SLC0_RX_AUTO_WRBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_AUTO_WRBACK`"]
pub struct SLC_SLC0_RX_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_AUTO_WRBACK_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_LOOP_TEST`"]
pub type SLC_SLC0_RX_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_LOOP_TEST`"]
pub struct SLC_SLC0_RX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_LOOP_TEST_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_LOOP_TEST`"]
pub type SLC_SLC0_TX_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_LOOP_TEST`"]
pub struct SLC_SLC0_TX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_LOOP_TEST_W<'a> {
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
#[doc = "Reader of field `SLC_AHBM_RST`"]
pub type SLC_AHBM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_AHBM_RST`"]
pub struct SLC_AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_AHBM_RST_W<'a> {
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
#[doc = "Reader of field `SLC_AHBM_FIFO_RST`"]
pub type SLC_AHBM_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_AHBM_FIFO_RST`"]
pub struct SLC_AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_AHBM_FIFO_RST_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_RST`"]
pub type SLC_SLC0_RX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_RST`"]
pub struct SLC_SLC0_RX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_RST_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_RST`"]
pub type SLC_SLC0_TX_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_RST`"]
pub struct SLC_SLC0_TX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_RST_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_slc1_token_sel(&self) -> SLC_SLC1_TOKEN_SEL_R {
        SLC_SLC1_TOKEN_SEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc_slc1_token_auto_clr(&self) -> SLC_SLC1_TOKEN_AUTO_CLR_R {
        SLC_SLC1_TOKEN_AUTO_CLR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc_slc1_txdata_burst_en(&self) -> SLC_SLC1_TXDATA_BURST_EN_R {
        SLC_SLC1_TXDATA_BURST_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc_slc1_txdscr_burst_en(&self) -> SLC_SLC1_TXDSCR_BURST_EN_R {
        SLC_SLC1_TXDSCR_BURST_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc_slc1_txlink_auto_ret(&self) -> SLC_SLC1_TXLINK_AUTO_RET_R {
        SLC_SLC1_TXLINK_AUTO_RET_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_auto_ret(&self) -> SLC_SLC1_RXLINK_AUTO_RET_R {
        SLC_SLC1_RXLINK_AUTO_RET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc_slc1_rxdata_burst_en(&self) -> SLC_SLC1_RXDATA_BURST_EN_R {
        SLC_SLC1_RXDATA_BURST_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc_slc1_rxdscr_burst_en(&self) -> SLC_SLC1_RXDSCR_BURST_EN_R {
        SLC_SLC1_RXDSCR_BURST_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc_slc1_rx_no_restart_clr(&self) -> SLC_SLC1_RX_NO_RESTART_CLR_R {
        SLC_SLC1_RX_NO_RESTART_CLR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc_slc1_rx_auto_wrback(&self) -> SLC_SLC1_RX_AUTO_WRBACK_R {
        SLC_SLC1_RX_AUTO_WRBACK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_slc1_rx_loop_test(&self) -> SLC_SLC1_RX_LOOP_TEST_R {
        SLC_SLC1_RX_LOOP_TEST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_tx_loop_test(&self) -> SLC_SLC1_TX_LOOP_TEST_R {
        SLC_SLC1_TX_LOOP_TEST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc1_wr_retry_mask_en(&self) -> SLC_SLC1_WR_RETRY_MASK_EN_R {
        SLC_SLC1_WR_RETRY_MASK_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc0_wr_retry_mask_en(&self) -> SLC_SLC0_WR_RETRY_MASK_EN_R {
        SLC_SLC0_WR_RETRY_MASK_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc1_rx_rst(&self) -> SLC_SLC1_RX_RST_R {
        SLC_SLC1_RX_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_tx_rst(&self) -> SLC_SLC1_TX_RST_R {
        SLC_SLC1_TX_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_slc0_token_sel(&self) -> SLC_SLC0_TOKEN_SEL_R {
        SLC_SLC0_TOKEN_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc0_token_auto_clr(&self) -> SLC_SLC0_TOKEN_AUTO_CLR_R {
        SLC_SLC0_TOKEN_AUTO_CLR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc0_txdata_burst_en(&self) -> SLC_SLC0_TXDATA_BURST_EN_R {
        SLC_SLC0_TXDATA_BURST_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc0_txdscr_burst_en(&self) -> SLC_SLC0_TXDSCR_BURST_EN_R {
        SLC_SLC0_TXDSCR_BURST_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_slc0_txlink_auto_ret(&self) -> SLC_SLC0_TXLINK_AUTO_RET_R {
        SLC_SLC0_TXLINK_AUTO_RET_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_slc0_rxlink_auto_ret(&self) -> SLC_SLC0_RXLINK_AUTO_RET_R {
        SLC_SLC0_RXLINK_AUTO_RET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_slc0_rxdata_burst_en(&self) -> SLC_SLC0_RXDATA_BURST_EN_R {
        SLC_SLC0_RXDATA_BURST_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_slc0_rxdscr_burst_en(&self) -> SLC_SLC0_RXDSCR_BURST_EN_R {
        SLC_SLC0_RXDSCR_BURST_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_slc0_rx_no_restart_clr(&self) -> SLC_SLC0_RX_NO_RESTART_CLR_R {
        SLC_SLC0_RX_NO_RESTART_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_slc0_rx_auto_wrback(&self) -> SLC_SLC0_RX_AUTO_WRBACK_R {
        SLC_SLC0_RX_AUTO_WRBACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_slc0_rx_loop_test(&self) -> SLC_SLC0_RX_LOOP_TEST_R {
        SLC_SLC0_RX_LOOP_TEST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_slc0_tx_loop_test(&self) -> SLC_SLC0_TX_LOOP_TEST_R {
        SLC_SLC0_TX_LOOP_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_ahbm_rst(&self) -> SLC_AHBM_RST_R {
        SLC_AHBM_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_ahbm_fifo_rst(&self) -> SLC_AHBM_FIFO_RST_R {
        SLC_AHBM_FIFO_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_slc0_rx_rst(&self) -> SLC_SLC0_RX_RST_R {
        SLC_SLC0_RX_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_slc0_tx_rst(&self) -> SLC_SLC0_TX_RST_R {
        SLC_SLC0_TX_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_slc1_token_sel(&mut self) -> SLC_SLC1_TOKEN_SEL_W {
        SLC_SLC1_TOKEN_SEL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc_slc1_token_auto_clr(&mut self) -> SLC_SLC1_TOKEN_AUTO_CLR_W {
        SLC_SLC1_TOKEN_AUTO_CLR_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc_slc1_txdata_burst_en(&mut self) -> SLC_SLC1_TXDATA_BURST_EN_W {
        SLC_SLC1_TXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc_slc1_txdscr_burst_en(&mut self) -> SLC_SLC1_TXDSCR_BURST_EN_W {
        SLC_SLC1_TXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc_slc1_txlink_auto_ret(&mut self) -> SLC_SLC1_TXLINK_AUTO_RET_W {
        SLC_SLC1_TXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_auto_ret(&mut self) -> SLC_SLC1_RXLINK_AUTO_RET_W {
        SLC_SLC1_RXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc_slc1_rxdata_burst_en(&mut self) -> SLC_SLC1_RXDATA_BURST_EN_W {
        SLC_SLC1_RXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc_slc1_rxdscr_burst_en(&mut self) -> SLC_SLC1_RXDSCR_BURST_EN_W {
        SLC_SLC1_RXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc_slc1_rx_no_restart_clr(&mut self) -> SLC_SLC1_RX_NO_RESTART_CLR_W {
        SLC_SLC1_RX_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc_slc1_rx_auto_wrback(&mut self) -> SLC_SLC1_RX_AUTO_WRBACK_W {
        SLC_SLC1_RX_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_slc1_rx_loop_test(&mut self) -> SLC_SLC1_RX_LOOP_TEST_W {
        SLC_SLC1_RX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_tx_loop_test(&mut self) -> SLC_SLC1_TX_LOOP_TEST_W {
        SLC_SLC1_TX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc1_wr_retry_mask_en(&mut self) -> SLC_SLC1_WR_RETRY_MASK_EN_W {
        SLC_SLC1_WR_RETRY_MASK_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc0_wr_retry_mask_en(&mut self) -> SLC_SLC0_WR_RETRY_MASK_EN_W {
        SLC_SLC0_WR_RETRY_MASK_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc1_rx_rst(&mut self) -> SLC_SLC1_RX_RST_W {
        SLC_SLC1_RX_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_tx_rst(&mut self) -> SLC_SLC1_TX_RST_W {
        SLC_SLC1_TX_RST_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_slc0_token_sel(&mut self) -> SLC_SLC0_TOKEN_SEL_W {
        SLC_SLC0_TOKEN_SEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc0_token_auto_clr(&mut self) -> SLC_SLC0_TOKEN_AUTO_CLR_W {
        SLC_SLC0_TOKEN_AUTO_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc0_txdata_burst_en(&mut self) -> SLC_SLC0_TXDATA_BURST_EN_W {
        SLC_SLC0_TXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc0_txdscr_burst_en(&mut self) -> SLC_SLC0_TXDSCR_BURST_EN_W {
        SLC_SLC0_TXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_slc0_txlink_auto_ret(&mut self) -> SLC_SLC0_TXLINK_AUTO_RET_W {
        SLC_SLC0_TXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_slc0_rxlink_auto_ret(&mut self) -> SLC_SLC0_RXLINK_AUTO_RET_W {
        SLC_SLC0_RXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_slc0_rxdata_burst_en(&mut self) -> SLC_SLC0_RXDATA_BURST_EN_W {
        SLC_SLC0_RXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_slc0_rxdscr_burst_en(&mut self) -> SLC_SLC0_RXDSCR_BURST_EN_W {
        SLC_SLC0_RXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_slc0_rx_no_restart_clr(&mut self) -> SLC_SLC0_RX_NO_RESTART_CLR_W {
        SLC_SLC0_RX_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_slc0_rx_auto_wrback(&mut self) -> SLC_SLC0_RX_AUTO_WRBACK_W {
        SLC_SLC0_RX_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_slc0_rx_loop_test(&mut self) -> SLC_SLC0_RX_LOOP_TEST_W {
        SLC_SLC0_RX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_slc0_tx_loop_test(&mut self) -> SLC_SLC0_TX_LOOP_TEST_W {
        SLC_SLC0_TX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_ahbm_rst(&mut self) -> SLC_AHBM_RST_W {
        SLC_AHBM_RST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_ahbm_fifo_rst(&mut self) -> SLC_AHBM_FIFO_RST_W {
        SLC_AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_slc0_rx_rst(&mut self) -> SLC_SLC0_RX_RST_W {
        SLC_SLC0_RX_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_slc0_tx_rst(&mut self) -> SLC_SLC0_TX_RST_W {
        SLC_SLC0_TX_RST_W { w: self }
    }
}
