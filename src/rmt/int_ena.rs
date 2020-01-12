#[doc = "Reader of register INT_ENA"]
pub type R = crate::R<u32, super::INT_ENA>;
#[doc = "Writer for register INT_ENA"]
pub type W = crate::W<u32, super::INT_ENA>;
#[doc = "Register INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH7_TX_THR_EVENT_INT_ENA`"]
pub type CH7_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7_TX_THR_EVENT_INT_ENA`"]
pub struct CH7_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH6_TX_THR_EVENT_INT_ENA`"]
pub type CH6_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6_TX_THR_EVENT_INT_ENA`"]
pub struct CH6_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH5_TX_THR_EVENT_INT_ENA`"]
pub type CH5_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5_TX_THR_EVENT_INT_ENA`"]
pub struct CH5_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH4_TX_THR_EVENT_INT_ENA`"]
pub type CH4_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4_TX_THR_EVENT_INT_ENA`"]
pub struct CH4_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH3_TX_THR_EVENT_INT_ENA`"]
pub type CH3_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_TX_THR_EVENT_INT_ENA`"]
pub struct CH3_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH2_TX_THR_EVENT_INT_ENA`"]
pub type CH2_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_TX_THR_EVENT_INT_ENA`"]
pub struct CH2_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH1_TX_THR_EVENT_INT_ENA`"]
pub type CH1_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_TX_THR_EVENT_INT_ENA`"]
pub struct CH1_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH0_TX_THR_EVENT_INT_ENA`"]
pub type CH0_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_TX_THR_EVENT_INT_ENA`"]
pub struct CH0_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH7_ERR_INT_ENA`"]
pub type CH7_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7_ERR_INT_ENA`"]
pub struct CH7_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH7_RX_END_INT_ENA`"]
pub type CH7_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7_RX_END_INT_ENA`"]
pub struct CH7_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH7_TX_END_INT_ENA`"]
pub type CH7_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7_TX_END_INT_ENA`"]
pub struct CH7_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH6_ERR_INT_ENA`"]
pub type CH6_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6_ERR_INT_ENA`"]
pub struct CH6_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH6_RX_END_INT_ENA`"]
pub type CH6_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6_RX_END_INT_ENA`"]
pub struct CH6_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH6_TX_END_INT_ENA`"]
pub type CH6_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6_TX_END_INT_ENA`"]
pub struct CH6_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH5_ERR_INT_ENA`"]
pub type CH5_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5_ERR_INT_ENA`"]
pub struct CH5_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH5_RX_END_INT_ENA`"]
pub type CH5_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5_RX_END_INT_ENA`"]
pub struct CH5_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH5_TX_END_INT_ENA`"]
pub type CH5_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5_TX_END_INT_ENA`"]
pub struct CH5_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH4_ERR_INT_ENA`"]
pub type CH4_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4_ERR_INT_ENA`"]
pub struct CH4_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH4_RX_END_INT_ENA`"]
pub type CH4_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4_RX_END_INT_ENA`"]
pub struct CH4_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH4_TX_END_INT_ENA`"]
pub type CH4_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4_TX_END_INT_ENA`"]
pub struct CH4_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH3_ERR_INT_ENA`"]
pub type CH3_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_ERR_INT_ENA`"]
pub struct CH3_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH3_RX_END_INT_ENA`"]
pub type CH3_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_RX_END_INT_ENA`"]
pub struct CH3_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH3_TX_END_INT_ENA`"]
pub type CH3_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_TX_END_INT_ENA`"]
pub struct CH3_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH2_ERR_INT_ENA`"]
pub type CH2_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_ERR_INT_ENA`"]
pub struct CH2_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH2_RX_END_INT_ENA`"]
pub type CH2_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_RX_END_INT_ENA`"]
pub struct CH2_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH2_TX_END_INT_ENA`"]
pub type CH2_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_TX_END_INT_ENA`"]
pub struct CH2_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH1_ERR_INT_ENA`"]
pub type CH1_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_ERR_INT_ENA`"]
pub struct CH1_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH1_RX_END_INT_ENA`"]
pub type CH1_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_RX_END_INT_ENA`"]
pub struct CH1_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH1_TX_END_INT_ENA`"]
pub type CH1_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_TX_END_INT_ENA`"]
pub struct CH1_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH0_ERR_INT_ENA`"]
pub type CH0_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_ERR_INT_ENA`"]
pub struct CH0_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH0_RX_END_INT_ENA`"]
pub type CH0_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_RX_END_INT_ENA`"]
pub struct CH0_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `CH0_TX_END_INT_ENA`"]
pub type CH0_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_TX_END_INT_ENA`"]
pub struct CH0_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_END_INT_ENA_W<'a> {
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
    #[doc = "Bit 31 - Set this bit to enable rmt_ch7_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch7_tx_thr_event_int_ena(&self) -> CH7_TX_THR_EVENT_INT_ENA_R {
        CH7_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable rmt_ch6_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch6_tx_thr_event_int_ena(&self) -> CH6_TX_THR_EVENT_INT_ENA_R {
        CH6_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Set this bit to enable rmt_ch5_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch5_tx_thr_event_int_ena(&self) -> CH5_TX_THR_EVENT_INT_ENA_R {
        CH5_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable rmt_ch4_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch4_tx_thr_event_int_ena(&self) -> CH4_TX_THR_EVENT_INT_ENA_R {
        CH4_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable rmt_ch3_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&self) -> CH3_TX_THR_EVENT_INT_ENA_R {
        CH3_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable rmt_ch2_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&self) -> CH2_TX_THR_EVENT_INT_ENA_R {
        CH2_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable rmt_ch1_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&self) -> CH1_TX_THR_EVENT_INT_ENA_R {
        CH1_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable rmt_ch0_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&self) -> CH0_TX_THR_EVENT_INT_ENA_R {
        CH0_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable rmt_ch7_err_int_st."]
    #[inline(always)]
    pub fn ch7_err_int_ena(&self) -> CH7_ERR_INT_ENA_R {
        CH7_ERR_INT_ENA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable rmt_ch7_rx_end_int_st."]
    #[inline(always)]
    pub fn ch7_rx_end_int_ena(&self) -> CH7_RX_END_INT_ENA_R {
        CH7_RX_END_INT_ENA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable rmt_ch7_tx_end_int_st."]
    #[inline(always)]
    pub fn ch7_tx_end_int_ena(&self) -> CH7_TX_END_INT_ENA_R {
        CH7_TX_END_INT_ENA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable rmt_ch6_err_int_st."]
    #[inline(always)]
    pub fn ch6_err_int_ena(&self) -> CH6_ERR_INT_ENA_R {
        CH6_ERR_INT_ENA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable rmt_ch6_rx_end_int_st."]
    #[inline(always)]
    pub fn ch6_rx_end_int_ena(&self) -> CH6_RX_END_INT_ENA_R {
        CH6_RX_END_INT_ENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set this bit to enable rmt_ch6_tx_end_int_st."]
    #[inline(always)]
    pub fn ch6_tx_end_int_ena(&self) -> CH6_TX_END_INT_ENA_R {
        CH6_TX_END_INT_ENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable rmt_ch5_err_int_st."]
    #[inline(always)]
    pub fn ch5_err_int_ena(&self) -> CH5_ERR_INT_ENA_R {
        CH5_ERR_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable rmt_ch5_rx_end_int_st."]
    #[inline(always)]
    pub fn ch5_rx_end_int_ena(&self) -> CH5_RX_END_INT_ENA_R {
        CH5_RX_END_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable rmt_ch5_tx_end_int_st."]
    #[inline(always)]
    pub fn ch5_tx_end_int_ena(&self) -> CH5_TX_END_INT_ENA_R {
        CH5_TX_END_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable rmt_ch4_err_int_st."]
    #[inline(always)]
    pub fn ch4_err_int_ena(&self) -> CH4_ERR_INT_ENA_R {
        CH4_ERR_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable rmt_ch4_rx_end_int_st."]
    #[inline(always)]
    pub fn ch4_rx_end_int_ena(&self) -> CH4_RX_END_INT_ENA_R {
        CH4_RX_END_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable rmt_ch4_tx_end_int_st."]
    #[inline(always)]
    pub fn ch4_tx_end_int_ena(&self) -> CH4_TX_END_INT_ENA_R {
        CH4_TX_END_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable rmt_ch3_err_int_st."]
    #[inline(always)]
    pub fn ch3_err_int_ena(&self) -> CH3_ERR_INT_ENA_R {
        CH3_ERR_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable rmt_ch3_rx_end_int_st."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&self) -> CH3_RX_END_INT_ENA_R {
        CH3_RX_END_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable rmt_ch3_tx_end_int_st."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&self) -> CH3_TX_END_INT_ENA_R {
        CH3_TX_END_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable rmt_ch2_err_int_st."]
    #[inline(always)]
    pub fn ch2_err_int_ena(&self) -> CH2_ERR_INT_ENA_R {
        CH2_ERR_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable rmt_ch2_rx_end_int_st."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&self) -> CH2_RX_END_INT_ENA_R {
        CH2_RX_END_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable rmt_ch2_tx_end_int_st."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&self) -> CH2_TX_END_INT_ENA_R {
        CH2_TX_END_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable rmt_ch1_err_int_st."]
    #[inline(always)]
    pub fn ch1_err_int_ena(&self) -> CH1_ERR_INT_ENA_R {
        CH1_ERR_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable rmt_ch1_rx_end_int_st."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&self) -> CH1_RX_END_INT_ENA_R {
        CH1_RX_END_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable rmt_ch1_tx_end_int_st."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&self) -> CH1_TX_END_INT_ENA_R {
        CH1_TX_END_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable rmt_ch0_err_int_st."]
    #[inline(always)]
    pub fn ch0_err_int_ena(&self) -> CH0_ERR_INT_ENA_R {
        CH0_ERR_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable rmt_ch0_rx_end_int_st."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&self) -> CH0_RX_END_INT_ENA_R {
        CH0_RX_END_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to enable rmt_ch0_tx_end_int_st."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&self) -> CH0_TX_END_INT_ENA_R {
        CH0_TX_END_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to enable rmt_ch7_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch7_tx_thr_event_int_ena(&mut self) -> CH7_TX_THR_EVENT_INT_ENA_W {
        CH7_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to enable rmt_ch6_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch6_tx_thr_event_int_ena(&mut self) -> CH6_TX_THR_EVENT_INT_ENA_W {
        CH6_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to enable rmt_ch5_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch5_tx_thr_event_int_ena(&mut self) -> CH5_TX_THR_EVENT_INT_ENA_W {
        CH5_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to enable rmt_ch4_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch4_tx_thr_event_int_ena(&mut self) -> CH4_TX_THR_EVENT_INT_ENA_W {
        CH4_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 27 - Set this bit to enable rmt_ch3_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&mut self) -> CH3_TX_THR_EVENT_INT_ENA_W {
        CH3_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 26 - Set this bit to enable rmt_ch2_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&mut self) -> CH2_TX_THR_EVENT_INT_ENA_W {
        CH2_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 25 - Set this bit to enable rmt_ch1_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&mut self) -> CH1_TX_THR_EVENT_INT_ENA_W {
        CH1_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 24 - Set this bit to enable rmt_ch0_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&mut self) -> CH0_TX_THR_EVENT_INT_ENA_W {
        CH0_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 23 - Set this bit to enable rmt_ch7_err_int_st."]
    #[inline(always)]
    pub fn ch7_err_int_ena(&mut self) -> CH7_ERR_INT_ENA_W {
        CH7_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 22 - Set this bit to enable rmt_ch7_rx_end_int_st."]
    #[inline(always)]
    pub fn ch7_rx_end_int_ena(&mut self) -> CH7_RX_END_INT_ENA_W {
        CH7_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to enable rmt_ch7_tx_end_int_st."]
    #[inline(always)]
    pub fn ch7_tx_end_int_ena(&mut self) -> CH7_TX_END_INT_ENA_W {
        CH7_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to enable rmt_ch6_err_int_st."]
    #[inline(always)]
    pub fn ch6_err_int_ena(&mut self) -> CH6_ERR_INT_ENA_W {
        CH6_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19 - Set this bit to enable rmt_ch6_rx_end_int_st."]
    #[inline(always)]
    pub fn ch6_rx_end_int_ena(&mut self) -> CH6_RX_END_INT_ENA_W {
        CH6_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18 - Set this bit to enable rmt_ch6_tx_end_int_st."]
    #[inline(always)]
    pub fn ch6_tx_end_int_ena(&mut self) -> CH6_TX_END_INT_ENA_W {
        CH6_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to enable rmt_ch5_err_int_st."]
    #[inline(always)]
    pub fn ch5_err_int_ena(&mut self) -> CH5_ERR_INT_ENA_W {
        CH5_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to enable rmt_ch5_rx_end_int_st."]
    #[inline(always)]
    pub fn ch5_rx_end_int_ena(&mut self) -> CH5_RX_END_INT_ENA_W {
        CH5_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - Set this bit to enable rmt_ch5_tx_end_int_st."]
    #[inline(always)]
    pub fn ch5_tx_end_int_ena(&mut self) -> CH5_TX_END_INT_ENA_W {
        CH5_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to enable rmt_ch4_err_int_st."]
    #[inline(always)]
    pub fn ch4_err_int_ena(&mut self) -> CH4_ERR_INT_ENA_W {
        CH4_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to enable rmt_ch4_rx_end_int_st."]
    #[inline(always)]
    pub fn ch4_rx_end_int_ena(&mut self) -> CH4_RX_END_INT_ENA_W {
        CH4_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to enable rmt_ch4_tx_end_int_st."]
    #[inline(always)]
    pub fn ch4_tx_end_int_ena(&mut self) -> CH4_TX_END_INT_ENA_W {
        CH4_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to enable rmt_ch3_err_int_st."]
    #[inline(always)]
    pub fn ch3_err_int_ena(&mut self) -> CH3_ERR_INT_ENA_W {
        CH3_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to enable rmt_ch3_rx_end_int_st."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&mut self) -> CH3_RX_END_INT_ENA_W {
        CH3_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to enable rmt_ch3_tx_end_int_st."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&mut self) -> CH3_TX_END_INT_ENA_W {
        CH3_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to enable rmt_ch2_err_int_st."]
    #[inline(always)]
    pub fn ch2_err_int_ena(&mut self) -> CH2_ERR_INT_ENA_W {
        CH2_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to enable rmt_ch2_rx_end_int_st."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&mut self) -> CH2_RX_END_INT_ENA_W {
        CH2_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to enable rmt_ch2_tx_end_int_st."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&mut self) -> CH2_TX_END_INT_ENA_W {
        CH2_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to enable rmt_ch1_err_int_st."]
    #[inline(always)]
    pub fn ch1_err_int_ena(&mut self) -> CH1_ERR_INT_ENA_W {
        CH1_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to enable rmt_ch1_rx_end_int_st."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&mut self) -> CH1_RX_END_INT_ENA_W {
        CH1_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable rmt_ch1_tx_end_int_st."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&mut self) -> CH1_TX_END_INT_ENA_W {
        CH1_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to enable rmt_ch0_err_int_st."]
    #[inline(always)]
    pub fn ch0_err_int_ena(&mut self) -> CH0_ERR_INT_ENA_W {
        CH0_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable rmt_ch0_rx_end_int_st."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&mut self) -> CH0_RX_END_INT_ENA_W {
        CH0_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to enable rmt_ch0_tx_end_int_st."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&mut self) -> CH0_TX_END_INT_ENA_W {
        CH0_TX_END_INT_ENA_W { w: self }
    }
}
