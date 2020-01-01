#[doc = "Reader of register 0INT_ENA1"]
pub type R = crate::R<u32, super::_0INT_ENA1>;
#[doc = "Writer for register 0INT_ENA1"]
pub type W = crate::W<u32, super::_0INT_ENA1>;
#[doc = "Register 0INT_ENA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::_0INT_ENA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC0_RX_QUICK_EOF_INT_ENA1`"]
pub type SLC_SLC0_RX_QUICK_EOF_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_QUICK_EOF_INT_ENA1`"]
pub struct SLC_SLC0_RX_QUICK_EOF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_QUICK_EOF_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_CMD_DTC_INT_ENA1`"]
pub type SLC_CMD_DTC_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_CMD_DTC_INT_ENA1`"]
pub struct SLC_CMD_DTC_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_CMD_DTC_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_ERR_EOF_INT_ENA1`"]
pub type SLC_SLC0_TX_ERR_EOF_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_ERR_EOF_INT_ENA1`"]
pub struct SLC_SLC0_TX_ERR_EOF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_ERR_EOF_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_WR_RETRY_DONE_INT_ENA1`"]
pub type SLC_SLC0_WR_RETRY_DONE_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_WR_RETRY_DONE_INT_ENA1`"]
pub struct SLC_SLC0_WR_RETRY_DONE_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_WR_RETRY_DONE_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_HOST_RD_ACK_INT_ENA1`"]
pub type SLC_SLC0_HOST_RD_ACK_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_HOST_RD_ACK_INT_ENA1`"]
pub struct SLC_SLC0_HOST_RD_ACK_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_HOST_RD_ACK_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1`"]
pub type SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1`"]
pub struct SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_DSCR_ERR_INT_ENA1`"]
pub type SLC_SLC0_RX_DSCR_ERR_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_DSCR_ERR_INT_ENA1`"]
pub struct SLC_SLC0_RX_DSCR_ERR_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_DSCR_ERR_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_DSCR_ERR_INT_ENA1`"]
pub type SLC_SLC0_TX_DSCR_ERR_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_DSCR_ERR_INT_ENA1`"]
pub struct SLC_SLC0_TX_DSCR_ERR_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_DSCR_ERR_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TOHOST_INT_ENA1`"]
pub type SLC_SLC0_TOHOST_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TOHOST_INT_ENA1`"]
pub struct SLC_SLC0_TOHOST_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TOHOST_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_EOF_INT_ENA1`"]
pub type SLC_SLC0_RX_EOF_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_EOF_INT_ENA1`"]
pub struct SLC_SLC0_RX_EOF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_EOF_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_DONE_INT_ENA1`"]
pub type SLC_SLC0_RX_DONE_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_DONE_INT_ENA1`"]
pub struct SLC_SLC0_RX_DONE_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_DONE_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_SUC_EOF_INT_ENA1`"]
pub type SLC_SLC0_TX_SUC_EOF_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_SUC_EOF_INT_ENA1`"]
pub struct SLC_SLC0_TX_SUC_EOF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_SUC_EOF_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_DONE_INT_ENA1`"]
pub type SLC_SLC0_TX_DONE_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_DONE_INT_ENA1`"]
pub struct SLC_SLC0_TX_DONE_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_DONE_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TOKEN1_1TO0_INT_ENA1`"]
pub type SLC_SLC0_TOKEN1_1TO0_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TOKEN1_1TO0_INT_ENA1`"]
pub struct SLC_SLC0_TOKEN1_1TO0_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TOKEN1_1TO0_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TOKEN0_1TO0_INT_ENA1`"]
pub type SLC_SLC0_TOKEN0_1TO0_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TOKEN0_1TO0_INT_ENA1`"]
pub struct SLC_SLC0_TOKEN0_1TO0_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TOKEN0_1TO0_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_OVF_INT_ENA1`"]
pub type SLC_SLC0_TX_OVF_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_OVF_INT_ENA1`"]
pub struct SLC_SLC0_TX_OVF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_OVF_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_UDF_INT_ENA1`"]
pub type SLC_SLC0_RX_UDF_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_UDF_INT_ENA1`"]
pub struct SLC_SLC0_RX_UDF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_UDF_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_START_INT_ENA1`"]
pub type SLC_SLC0_TX_START_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_START_INT_ENA1`"]
pub struct SLC_SLC0_TX_START_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_START_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_START_INT_ENA1`"]
pub type SLC_SLC0_RX_START_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_START_INT_ENA1`"]
pub struct SLC_SLC0_RX_START_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_START_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT7_INT_ENA1`"]
pub type SLC_FRHOST_BIT7_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT7_INT_ENA1`"]
pub struct SLC_FRHOST_BIT7_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT7_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT6_INT_ENA1`"]
pub type SLC_FRHOST_BIT6_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT6_INT_ENA1`"]
pub struct SLC_FRHOST_BIT6_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT6_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT5_INT_ENA1`"]
pub type SLC_FRHOST_BIT5_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT5_INT_ENA1`"]
pub struct SLC_FRHOST_BIT5_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT5_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT4_INT_ENA1`"]
pub type SLC_FRHOST_BIT4_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT4_INT_ENA1`"]
pub struct SLC_FRHOST_BIT4_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT4_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT3_INT_ENA1`"]
pub type SLC_FRHOST_BIT3_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT3_INT_ENA1`"]
pub struct SLC_FRHOST_BIT3_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT3_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT2_INT_ENA1`"]
pub type SLC_FRHOST_BIT2_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT2_INT_ENA1`"]
pub struct SLC_FRHOST_BIT2_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT2_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT1_INT_ENA1`"]
pub type SLC_FRHOST_BIT1_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT1_INT_ENA1`"]
pub struct SLC_FRHOST_BIT1_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT1_INT_ENA1_W<'a> {
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
#[doc = "Reader of field `SLC_FRHOST_BIT0_INT_ENA1`"]
pub type SLC_FRHOST_BIT0_INT_ENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_FRHOST_BIT0_INT_ENA1`"]
pub struct SLC_FRHOST_BIT0_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FRHOST_BIT0_INT_ENA1_W<'a> {
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
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc_slc0_rx_quick_eof_int_ena1(&self) -> SLC_SLC0_RX_QUICK_EOF_INT_ENA1_R {
        SLC_SLC0_RX_QUICK_EOF_INT_ENA1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc_cmd_dtc_int_ena1(&self) -> SLC_CMD_DTC_INT_ENA1_R {
        SLC_CMD_DTC_INT_ENA1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc_slc0_tx_err_eof_int_ena1(&self) -> SLC_SLC0_TX_ERR_EOF_INT_ENA1_R {
        SLC_SLC0_TX_ERR_EOF_INT_ENA1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc_slc0_wr_retry_done_int_ena1(&self) -> SLC_SLC0_WR_RETRY_DONE_INT_ENA1_R {
        SLC_SLC0_WR_RETRY_DONE_INT_ENA1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc_slc0_host_rd_ack_int_ena1(&self) -> SLC_SLC0_HOST_RD_ACK_INT_ENA1_R {
        SLC_SLC0_HOST_RD_ACK_INT_ENA1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_slc0_tx_dscr_empty_int_ena1(&self) -> SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1_R {
        SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc0_rx_dscr_err_int_ena1(&self) -> SLC_SLC0_RX_DSCR_ERR_INT_ENA1_R {
        SLC_SLC0_RX_DSCR_ERR_INT_ENA1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc0_tx_dscr_err_int_ena1(&self) -> SLC_SLC0_TX_DSCR_ERR_INT_ENA1_R {
        SLC_SLC0_TX_DSCR_ERR_INT_ENA1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc0_tohost_int_ena1(&self) -> SLC_SLC0_TOHOST_INT_ENA1_R {
        SLC_SLC0_TOHOST_INT_ENA1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc0_rx_eof_int_ena1(&self) -> SLC_SLC0_RX_EOF_INT_ENA1_R {
        SLC_SLC0_RX_EOF_INT_ENA1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc0_rx_done_int_ena1(&self) -> SLC_SLC0_RX_DONE_INT_ENA1_R {
        SLC_SLC0_RX_DONE_INT_ENA1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_slc0_tx_suc_eof_int_ena1(&self) -> SLC_SLC0_TX_SUC_EOF_INT_ENA1_R {
        SLC_SLC0_TX_SUC_EOF_INT_ENA1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc0_tx_done_int_ena1(&self) -> SLC_SLC0_TX_DONE_INT_ENA1_R {
        SLC_SLC0_TX_DONE_INT_ENA1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc0_token1_1to0_int_ena1(&self) -> SLC_SLC0_TOKEN1_1TO0_INT_ENA1_R {
        SLC_SLC0_TOKEN1_1TO0_INT_ENA1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc0_token0_1to0_int_ena1(&self) -> SLC_SLC0_TOKEN0_1TO0_INT_ENA1_R {
        SLC_SLC0_TOKEN0_1TO0_INT_ENA1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_slc0_tx_ovf_int_ena1(&self) -> SLC_SLC0_TX_OVF_INT_ENA1_R {
        SLC_SLC0_TX_OVF_INT_ENA1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_slc0_rx_udf_int_ena1(&self) -> SLC_SLC0_RX_UDF_INT_ENA1_R {
        SLC_SLC0_RX_UDF_INT_ENA1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_slc0_tx_start_int_ena1(&self) -> SLC_SLC0_TX_START_INT_ENA1_R {
        SLC_SLC0_TX_START_INT_ENA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_slc0_rx_start_int_ena1(&self) -> SLC_SLC0_RX_START_INT_ENA1_R {
        SLC_SLC0_RX_START_INT_ENA1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_frhost_bit7_int_ena1(&self) -> SLC_FRHOST_BIT7_INT_ENA1_R {
        SLC_FRHOST_BIT7_INT_ENA1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_frhost_bit6_int_ena1(&self) -> SLC_FRHOST_BIT6_INT_ENA1_R {
        SLC_FRHOST_BIT6_INT_ENA1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_frhost_bit5_int_ena1(&self) -> SLC_FRHOST_BIT5_INT_ENA1_R {
        SLC_FRHOST_BIT5_INT_ENA1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_frhost_bit4_int_ena1(&self) -> SLC_FRHOST_BIT4_INT_ENA1_R {
        SLC_FRHOST_BIT4_INT_ENA1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_frhost_bit3_int_ena1(&self) -> SLC_FRHOST_BIT3_INT_ENA1_R {
        SLC_FRHOST_BIT3_INT_ENA1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_frhost_bit2_int_ena1(&self) -> SLC_FRHOST_BIT2_INT_ENA1_R {
        SLC_FRHOST_BIT2_INT_ENA1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_frhost_bit1_int_ena1(&self) -> SLC_FRHOST_BIT1_INT_ENA1_R {
        SLC_FRHOST_BIT1_INT_ENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_frhost_bit0_int_ena1(&self) -> SLC_FRHOST_BIT0_INT_ENA1_R {
        SLC_FRHOST_BIT0_INT_ENA1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc_slc0_rx_quick_eof_int_ena1(&mut self) -> SLC_SLC0_RX_QUICK_EOF_INT_ENA1_W {
        SLC_SLC0_RX_QUICK_EOF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc_cmd_dtc_int_ena1(&mut self) -> SLC_CMD_DTC_INT_ENA1_W {
        SLC_CMD_DTC_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc_slc0_tx_err_eof_int_ena1(&mut self) -> SLC_SLC0_TX_ERR_EOF_INT_ENA1_W {
        SLC_SLC0_TX_ERR_EOF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc_slc0_wr_retry_done_int_ena1(&mut self) -> SLC_SLC0_WR_RETRY_DONE_INT_ENA1_W {
        SLC_SLC0_WR_RETRY_DONE_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc_slc0_host_rd_ack_int_ena1(&mut self) -> SLC_SLC0_HOST_RD_ACK_INT_ENA1_W {
        SLC_SLC0_HOST_RD_ACK_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_slc0_tx_dscr_empty_int_ena1(&mut self) -> SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1_W {
        SLC_SLC0_TX_DSCR_EMPTY_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc0_rx_dscr_err_int_ena1(&mut self) -> SLC_SLC0_RX_DSCR_ERR_INT_ENA1_W {
        SLC_SLC0_RX_DSCR_ERR_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc0_tx_dscr_err_int_ena1(&mut self) -> SLC_SLC0_TX_DSCR_ERR_INT_ENA1_W {
        SLC_SLC0_TX_DSCR_ERR_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc0_tohost_int_ena1(&mut self) -> SLC_SLC0_TOHOST_INT_ENA1_W {
        SLC_SLC0_TOHOST_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc0_rx_eof_int_ena1(&mut self) -> SLC_SLC0_RX_EOF_INT_ENA1_W {
        SLC_SLC0_RX_EOF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc0_rx_done_int_ena1(&mut self) -> SLC_SLC0_RX_DONE_INT_ENA1_W {
        SLC_SLC0_RX_DONE_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_slc0_tx_suc_eof_int_ena1(&mut self) -> SLC_SLC0_TX_SUC_EOF_INT_ENA1_W {
        SLC_SLC0_TX_SUC_EOF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc0_tx_done_int_ena1(&mut self) -> SLC_SLC0_TX_DONE_INT_ENA1_W {
        SLC_SLC0_TX_DONE_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_slc0_token1_1to0_int_ena1(&mut self) -> SLC_SLC0_TOKEN1_1TO0_INT_ENA1_W {
        SLC_SLC0_TOKEN1_1TO0_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc0_token0_1to0_int_ena1(&mut self) -> SLC_SLC0_TOKEN0_1TO0_INT_ENA1_W {
        SLC_SLC0_TOKEN0_1TO0_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_slc0_tx_ovf_int_ena1(&mut self) -> SLC_SLC0_TX_OVF_INT_ENA1_W {
        SLC_SLC0_TX_OVF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_slc0_rx_udf_int_ena1(&mut self) -> SLC_SLC0_RX_UDF_INT_ENA1_W {
        SLC_SLC0_RX_UDF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_slc0_tx_start_int_ena1(&mut self) -> SLC_SLC0_TX_START_INT_ENA1_W {
        SLC_SLC0_TX_START_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_slc0_rx_start_int_ena1(&mut self) -> SLC_SLC0_RX_START_INT_ENA1_W {
        SLC_SLC0_RX_START_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_frhost_bit7_int_ena1(&mut self) -> SLC_FRHOST_BIT7_INT_ENA1_W {
        SLC_FRHOST_BIT7_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_frhost_bit6_int_ena1(&mut self) -> SLC_FRHOST_BIT6_INT_ENA1_W {
        SLC_FRHOST_BIT6_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_frhost_bit5_int_ena1(&mut self) -> SLC_FRHOST_BIT5_INT_ENA1_W {
        SLC_FRHOST_BIT5_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_frhost_bit4_int_ena1(&mut self) -> SLC_FRHOST_BIT4_INT_ENA1_W {
        SLC_FRHOST_BIT4_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_frhost_bit3_int_ena1(&mut self) -> SLC_FRHOST_BIT3_INT_ENA1_W {
        SLC_FRHOST_BIT3_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_frhost_bit2_int_ena1(&mut self) -> SLC_FRHOST_BIT2_INT_ENA1_W {
        SLC_FRHOST_BIT2_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_frhost_bit1_int_ena1(&mut self) -> SLC_FRHOST_BIT1_INT_ENA1_W {
        SLC_FRHOST_BIT1_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_frhost_bit0_int_ena1(&mut self) -> SLC_FRHOST_BIT0_INT_ENA1_W {
        SLC_FRHOST_BIT0_INT_ENA1_W { w: self }
    }
}
