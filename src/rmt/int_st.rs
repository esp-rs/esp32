#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Writer for register INT_ST"]
pub type W = crate::W<u32, super::INT_ST>;
#[doc = "Register INT_ST `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_CH7_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH7_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH7_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH7_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH7_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH6_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH6_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH6_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH6_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH6_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH5_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH5_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH5_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH5_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH5_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH4_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH4_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH4_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH4_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH4_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH3_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH3_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH3_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH3_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH2_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH2_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH2_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH2_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH1_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH1_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH1_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH0_TX_THR_EVENT_INT_ST`"]
pub type RMT_CH0_TX_THR_EVENT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_TX_THR_EVENT_INT_ST`"]
pub struct RMT_CH0_TX_THR_EVENT_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_THR_EVENT_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH7_ERR_INT_ST`"]
pub type RMT_CH7_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH7_ERR_INT_ST`"]
pub struct RMT_CH7_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH7_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH7_RX_END_INT_ST`"]
pub type RMT_CH7_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH7_RX_END_INT_ST`"]
pub struct RMT_CH7_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH7_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH7_TX_END_INT_ST`"]
pub type RMT_CH7_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH7_TX_END_INT_ST`"]
pub struct RMT_CH7_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH7_TX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH6_ERR_INT_ST`"]
pub type RMT_CH6_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH6_ERR_INT_ST`"]
pub struct RMT_CH6_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH6_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH6_RX_END_INT_ST`"]
pub type RMT_CH6_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH6_RX_END_INT_ST`"]
pub struct RMT_CH6_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH6_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH6_TX_END_INT_ST`"]
pub type RMT_CH6_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH6_TX_END_INT_ST`"]
pub struct RMT_CH6_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH6_TX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH5_ERR_INT_ST`"]
pub type RMT_CH5_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH5_ERR_INT_ST`"]
pub struct RMT_CH5_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH5_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH5_RX_END_INT_ST`"]
pub type RMT_CH5_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH5_RX_END_INT_ST`"]
pub struct RMT_CH5_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH5_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH5_TX_END_INT_ST`"]
pub type RMT_CH5_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH5_TX_END_INT_ST`"]
pub struct RMT_CH5_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH5_TX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH4_ERR_INT_ST`"]
pub type RMT_CH4_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH4_ERR_INT_ST`"]
pub struct RMT_CH4_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH4_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH4_RX_END_INT_ST`"]
pub type RMT_CH4_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH4_RX_END_INT_ST`"]
pub struct RMT_CH4_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH4_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH4_TX_END_INT_ST`"]
pub type RMT_CH4_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH4_TX_END_INT_ST`"]
pub struct RMT_CH4_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH4_TX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH3_ERR_INT_ST`"]
pub type RMT_CH3_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH3_ERR_INT_ST`"]
pub struct RMT_CH3_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH3_RX_END_INT_ST`"]
pub type RMT_CH3_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH3_RX_END_INT_ST`"]
pub struct RMT_CH3_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH3_TX_END_INT_ST`"]
pub type RMT_CH3_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH3_TX_END_INT_ST`"]
pub struct RMT_CH3_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_TX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH2_ERR_INT_ST`"]
pub type RMT_CH2_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH2_ERR_INT_ST`"]
pub struct RMT_CH2_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH2_RX_END_INT_ST`"]
pub type RMT_CH2_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH2_RX_END_INT_ST`"]
pub struct RMT_CH2_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH2_TX_END_INT_ST`"]
pub type RMT_CH2_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH2_TX_END_INT_ST`"]
pub struct RMT_CH2_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_TX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH1_ERR_INT_ST`"]
pub type RMT_CH1_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_ERR_INT_ST`"]
pub struct RMT_CH1_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH1_RX_END_INT_ST`"]
pub type RMT_CH1_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_RX_END_INT_ST`"]
pub struct RMT_CH1_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH1_TX_END_INT_ST`"]
pub type RMT_CH1_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_TX_END_INT_ST`"]
pub struct RMT_CH1_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH0_ERR_INT_ST`"]
pub type RMT_CH0_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_ERR_INT_ST`"]
pub struct RMT_CH0_ERR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_ERR_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH0_RX_END_INT_ST`"]
pub type RMT_CH0_RX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_RX_END_INT_ST`"]
pub struct RMT_CH0_RX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_RX_END_INT_ST_W<'a> {
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
#[doc = "Reader of field `RMT_CH0_TX_END_INT_ST`"]
pub type RMT_CH0_TX_END_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_TX_END_INT_ST`"]
pub struct RMT_CH0_TX_END_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_END_INT_ST_W<'a> {
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
    #[doc = "Bit 31 - The interrupt state bit for channel 7's rmt_ch7_tx_thr_event_int_raw when mt_ch7_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_tx_thr_event_int_st(&self) -> RMT_CH7_TX_THR_EVENT_INT_ST_R {
        RMT_CH7_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - The interrupt state bit for channel 6's rmt_ch6_tx_thr_event_int_raw when mt_ch6_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_tx_thr_event_int_st(&self) -> RMT_CH6_TX_THR_EVENT_INT_ST_R {
        RMT_CH6_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The interrupt state bit for channel 5's rmt_ch5_tx_thr_event_int_raw when mt_ch5_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_tx_thr_event_int_st(&self) -> RMT_CH5_TX_THR_EVENT_INT_ST_R {
        RMT_CH5_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The interrupt state bit for channel 4's rmt_ch4_tx_thr_event_int_raw when mt_ch4_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_tx_thr_event_int_st(&self) -> RMT_CH4_TX_THR_EVENT_INT_ST_R {
        RMT_CH4_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The interrupt state bit for channel 3's rmt_ch3_tx_thr_event_int_raw when mt_ch3_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_tx_thr_event_int_st(&self) -> RMT_CH3_TX_THR_EVENT_INT_ST_R {
        RMT_CH3_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The interrupt state bit for channel 2's rmt_ch2_tx_thr_event_int_raw when mt_ch2_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_tx_thr_event_int_st(&self) -> RMT_CH2_TX_THR_EVENT_INT_ST_R {
        RMT_CH2_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The interrupt state bit for channel 1's rmt_ch1_tx_thr_event_int_raw when mt_ch1_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_tx_thr_event_int_st(&self) -> RMT_CH1_TX_THR_EVENT_INT_ST_R {
        RMT_CH1_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The interrupt state bit for channel 0's rmt_ch0_tx_thr_event_int_raw when mt_ch0_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch0_tx_thr_event_int_st(&self) -> RMT_CH0_TX_THR_EVENT_INT_ST_R {
        RMT_CH0_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The interrupt state bit for channel 7's rmt_ch7_err_int_raw when rmt_ch7_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_err_int_st(&self) -> RMT_CH7_ERR_INT_ST_R {
        RMT_CH7_ERR_INT_ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The interrupt state bit for channel 7's rmt_ch7_rx_end_int_raw when rmt_ch7_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_rx_end_int_st(&self) -> RMT_CH7_RX_END_INT_ST_R {
        RMT_CH7_RX_END_INT_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The interrupt state bit for channel 7's mt_ch7_tx_end_int_raw when mt_ch7_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_tx_end_int_st(&self) -> RMT_CH7_TX_END_INT_ST_R {
        RMT_CH7_TX_END_INT_ST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The interrupt state bit for channel 6's rmt_ch6_err_int_raw when rmt_ch6_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_err_int_st(&self) -> RMT_CH6_ERR_INT_ST_R {
        RMT_CH6_ERR_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The interrupt state bit for channel 6's rmt_ch6_rx_end_int_raw when rmt_ch6_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_rx_end_int_st(&self) -> RMT_CH6_RX_END_INT_ST_R {
        RMT_CH6_RX_END_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The interrupt state bit for channel 6's mt_ch6_tx_end_int_raw when mt_ch6_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_tx_end_int_st(&self) -> RMT_CH6_TX_END_INT_ST_R {
        RMT_CH6_TX_END_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The interrupt state bit for channel 5's rmt_ch5_err_int_raw when rmt_ch5_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_err_int_st(&self) -> RMT_CH5_ERR_INT_ST_R {
        RMT_CH5_ERR_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The interrupt state bit for channel 5's rmt_ch5_rx_end_int_raw when rmt_ch5_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_rx_end_int_st(&self) -> RMT_CH5_RX_END_INT_ST_R {
        RMT_CH5_RX_END_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The interrupt state bit for channel 5's mt_ch5_tx_end_int_raw when mt_ch5_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_tx_end_int_st(&self) -> RMT_CH5_TX_END_INT_ST_R {
        RMT_CH5_TX_END_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The interrupt state bit for channel 4's rmt_ch4_err_int_raw when rmt_ch4_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_err_int_st(&self) -> RMT_CH4_ERR_INT_ST_R {
        RMT_CH4_ERR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The interrupt state bit for channel 4's rmt_ch4_rx_end_int_raw when rmt_ch4_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_rx_end_int_st(&self) -> RMT_CH4_RX_END_INT_ST_R {
        RMT_CH4_RX_END_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The interrupt state bit for channel 4's mt_ch4_tx_end_int_raw when mt_ch4_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_tx_end_int_st(&self) -> RMT_CH4_TX_END_INT_ST_R {
        RMT_CH4_TX_END_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The interrupt state bit for channel 3's rmt_ch3_err_int_raw when rmt_ch3_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_err_int_st(&self) -> RMT_CH3_ERR_INT_ST_R {
        RMT_CH3_ERR_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The interrupt state bit for channel 3's rmt_ch3_rx_end_int_raw when rmt_ch3_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_rx_end_int_st(&self) -> RMT_CH3_RX_END_INT_ST_R {
        RMT_CH3_RX_END_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The interrupt state bit for channel 3's mt_ch3_tx_end_int_raw when mt_ch3_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_tx_end_int_st(&self) -> RMT_CH3_TX_END_INT_ST_R {
        RMT_CH3_TX_END_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The interrupt state bit for channel 2's rmt_ch2_err_int_raw when rmt_ch2_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_err_int_st(&self) -> RMT_CH2_ERR_INT_ST_R {
        RMT_CH2_ERR_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The interrupt state bit for channel 2's rmt_ch2_rx_end_int_raw when rmt_ch2_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_rx_end_int_st(&self) -> RMT_CH2_RX_END_INT_ST_R {
        RMT_CH2_RX_END_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The interrupt state bit for channel 2's mt_ch2_tx_end_int_raw when mt_ch2_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_tx_end_int_st(&self) -> RMT_CH2_TX_END_INT_ST_R {
        RMT_CH2_TX_END_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The interrupt state bit for channel 1's rmt_ch1_err_int_raw when rmt_ch1_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_err_int_st(&self) -> RMT_CH1_ERR_INT_ST_R {
        RMT_CH1_ERR_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt state bit for channel 1's rmt_ch1_rx_end_int_raw when rmt_ch1_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_rx_end_int_st(&self) -> RMT_CH1_RX_END_INT_ST_R {
        RMT_CH1_RX_END_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt state bit for channel 1's mt_ch1_tx_end_int_raw when mt_ch1_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_tx_end_int_st(&self) -> RMT_CH1_TX_END_INT_ST_R {
        RMT_CH1_TX_END_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt state bit for channel 0's rmt_ch0_err_int_raw when rmt_ch0_err_int_ena is set to 0."]
    #[inline(always)]
    pub fn rmt_ch0_err_int_st(&self) -> RMT_CH0_ERR_INT_ST_R {
        RMT_CH0_ERR_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt state bit for channel 0's rmt_ch0_rx_end_int_raw when rmt_ch0_rx_end_int_ena is set to 0."]
    #[inline(always)]
    pub fn rmt_ch0_rx_end_int_st(&self) -> RMT_CH0_RX_END_INT_ST_R {
        RMT_CH0_RX_END_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The interrupt state bit for channel 0's mt_ch0_tx_end_int_raw when mt_ch0_tx_end_int_ena is set to 0."]
    #[inline(always)]
    pub fn rmt_ch0_tx_end_int_st(&self) -> RMT_CH0_TX_END_INT_ST_R {
        RMT_CH0_TX_END_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - The interrupt state bit for channel 7's rmt_ch7_tx_thr_event_int_raw when mt_ch7_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_tx_thr_event_int_st(&mut self) -> RMT_CH7_TX_THR_EVENT_INT_ST_W {
        RMT_CH7_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 30 - The interrupt state bit for channel 6's rmt_ch6_tx_thr_event_int_raw when mt_ch6_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_tx_thr_event_int_st(&mut self) -> RMT_CH6_TX_THR_EVENT_INT_ST_W {
        RMT_CH6_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 29 - The interrupt state bit for channel 5's rmt_ch5_tx_thr_event_int_raw when mt_ch5_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_tx_thr_event_int_st(&mut self) -> RMT_CH5_TX_THR_EVENT_INT_ST_W {
        RMT_CH5_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 28 - The interrupt state bit for channel 4's rmt_ch4_tx_thr_event_int_raw when mt_ch4_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_tx_thr_event_int_st(&mut self) -> RMT_CH4_TX_THR_EVENT_INT_ST_W {
        RMT_CH4_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 27 - The interrupt state bit for channel 3's rmt_ch3_tx_thr_event_int_raw when mt_ch3_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_tx_thr_event_int_st(&mut self) -> RMT_CH3_TX_THR_EVENT_INT_ST_W {
        RMT_CH3_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 26 - The interrupt state bit for channel 2's rmt_ch2_tx_thr_event_int_raw when mt_ch2_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_tx_thr_event_int_st(&mut self) -> RMT_CH2_TX_THR_EVENT_INT_ST_W {
        RMT_CH2_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 25 - The interrupt state bit for channel 1's rmt_ch1_tx_thr_event_int_raw when mt_ch1_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_tx_thr_event_int_st(&mut self) -> RMT_CH1_TX_THR_EVENT_INT_ST_W {
        RMT_CH1_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 24 - The interrupt state bit for channel 0's rmt_ch0_tx_thr_event_int_raw when mt_ch0_tx_thr_event_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch0_tx_thr_event_int_st(&mut self) -> RMT_CH0_TX_THR_EVENT_INT_ST_W {
        RMT_CH0_TX_THR_EVENT_INT_ST_W { w: self }
    }
    #[doc = "Bit 23 - The interrupt state bit for channel 7's rmt_ch7_err_int_raw when rmt_ch7_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_err_int_st(&mut self) -> RMT_CH7_ERR_INT_ST_W {
        RMT_CH7_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 22 - The interrupt state bit for channel 7's rmt_ch7_rx_end_int_raw when rmt_ch7_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_rx_end_int_st(&mut self) -> RMT_CH7_RX_END_INT_ST_W {
        RMT_CH7_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 21 - The interrupt state bit for channel 7's mt_ch7_tx_end_int_raw when mt_ch7_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch7_tx_end_int_st(&mut self) -> RMT_CH7_TX_END_INT_ST_W {
        RMT_CH7_TX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 20 - The interrupt state bit for channel 6's rmt_ch6_err_int_raw when rmt_ch6_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_err_int_st(&mut self) -> RMT_CH6_ERR_INT_ST_W {
        RMT_CH6_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 19 - The interrupt state bit for channel 6's rmt_ch6_rx_end_int_raw when rmt_ch6_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_rx_end_int_st(&mut self) -> RMT_CH6_RX_END_INT_ST_W {
        RMT_CH6_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 18 - The interrupt state bit for channel 6's mt_ch6_tx_end_int_raw when mt_ch6_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch6_tx_end_int_st(&mut self) -> RMT_CH6_TX_END_INT_ST_W {
        RMT_CH6_TX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 17 - The interrupt state bit for channel 5's rmt_ch5_err_int_raw when rmt_ch5_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_err_int_st(&mut self) -> RMT_CH5_ERR_INT_ST_W {
        RMT_CH5_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 16 - The interrupt state bit for channel 5's rmt_ch5_rx_end_int_raw when rmt_ch5_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_rx_end_int_st(&mut self) -> RMT_CH5_RX_END_INT_ST_W {
        RMT_CH5_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt state bit for channel 5's mt_ch5_tx_end_int_raw when mt_ch5_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch5_tx_end_int_st(&mut self) -> RMT_CH5_TX_END_INT_ST_W {
        RMT_CH5_TX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 14 - The interrupt state bit for channel 4's rmt_ch4_err_int_raw when rmt_ch4_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_err_int_st(&mut self) -> RMT_CH4_ERR_INT_ST_W {
        RMT_CH4_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 13 - The interrupt state bit for channel 4's rmt_ch4_rx_end_int_raw when rmt_ch4_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_rx_end_int_st(&mut self) -> RMT_CH4_RX_END_INT_ST_W {
        RMT_CH4_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt state bit for channel 4's mt_ch4_tx_end_int_raw when mt_ch4_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch4_tx_end_int_st(&mut self) -> RMT_CH4_TX_END_INT_ST_W {
        RMT_CH4_TX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt state bit for channel 3's rmt_ch3_err_int_raw when rmt_ch3_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_err_int_st(&mut self) -> RMT_CH3_ERR_INT_ST_W {
        RMT_CH3_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt state bit for channel 3's rmt_ch3_rx_end_int_raw when rmt_ch3_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_rx_end_int_st(&mut self) -> RMT_CH3_RX_END_INT_ST_W {
        RMT_CH3_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt state bit for channel 3's mt_ch3_tx_end_int_raw when mt_ch3_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch3_tx_end_int_st(&mut self) -> RMT_CH3_TX_END_INT_ST_W {
        RMT_CH3_TX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt state bit for channel 2's rmt_ch2_err_int_raw when rmt_ch2_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_err_int_st(&mut self) -> RMT_CH2_ERR_INT_ST_W {
        RMT_CH2_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt state bit for channel 2's rmt_ch2_rx_end_int_raw when rmt_ch2_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_rx_end_int_st(&mut self) -> RMT_CH2_RX_END_INT_ST_W {
        RMT_CH2_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt state bit for channel 2's mt_ch2_tx_end_int_raw when mt_ch2_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch2_tx_end_int_st(&mut self) -> RMT_CH2_TX_END_INT_ST_W {
        RMT_CH2_TX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt state bit for channel 1's rmt_ch1_err_int_raw when rmt_ch1_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_err_int_st(&mut self) -> RMT_CH1_ERR_INT_ST_W {
        RMT_CH1_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt state bit for channel 1's rmt_ch1_rx_end_int_raw when rmt_ch1_rx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_rx_end_int_st(&mut self) -> RMT_CH1_RX_END_INT_ST_W {
        RMT_CH1_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt state bit for channel 1's mt_ch1_tx_end_int_raw when mt_ch1_tx_end_int_ena is set to 1."]
    #[inline(always)]
    pub fn rmt_ch1_tx_end_int_st(&mut self) -> RMT_CH1_TX_END_INT_ST_W {
        RMT_CH1_TX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt state bit for channel 0's rmt_ch0_err_int_raw when rmt_ch0_err_int_ena is set to 0."]
    #[inline(always)]
    pub fn rmt_ch0_err_int_st(&mut self) -> RMT_CH0_ERR_INT_ST_W {
        RMT_CH0_ERR_INT_ST_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt state bit for channel 0's rmt_ch0_rx_end_int_raw when rmt_ch0_rx_end_int_ena is set to 0."]
    #[inline(always)]
    pub fn rmt_ch0_rx_end_int_st(&mut self) -> RMT_CH0_RX_END_INT_ST_W {
        RMT_CH0_RX_END_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - The interrupt state bit for channel 0's mt_ch0_tx_end_int_raw when mt_ch0_tx_end_int_ena is set to 0."]
    #[inline(always)]
    pub fn rmt_ch0_tx_end_int_st(&mut self) -> RMT_CH0_TX_END_INT_ST_W {
        RMT_CH0_TX_END_INT_ST_W { w: self }
    }
}
