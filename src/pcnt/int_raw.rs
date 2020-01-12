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
#[doc = "Reader of field `CNT_THR_EVENT_U7_INT_RAW`"]
pub type CNT_THR_EVENT_U7_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U7_INT_RAW`"]
pub struct CNT_THR_EVENT_U7_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U7_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U6_INT_RAW`"]
pub type CNT_THR_EVENT_U6_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U6_INT_RAW`"]
pub struct CNT_THR_EVENT_U6_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U6_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U5_INT_RAW`"]
pub type CNT_THR_EVENT_U5_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U5_INT_RAW`"]
pub struct CNT_THR_EVENT_U5_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U5_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U4_INT_RAW`"]
pub type CNT_THR_EVENT_U4_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U4_INT_RAW`"]
pub struct CNT_THR_EVENT_U4_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U4_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U3_INT_RAW`"]
pub type CNT_THR_EVENT_U3_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U3_INT_RAW`"]
pub struct CNT_THR_EVENT_U3_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U3_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U2_INT_RAW`"]
pub type CNT_THR_EVENT_U2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U2_INT_RAW`"]
pub struct CNT_THR_EVENT_U2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U1_INT_RAW`"]
pub type CNT_THR_EVENT_U1_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U1_INT_RAW`"]
pub struct CNT_THR_EVENT_U1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U1_INT_RAW_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U0_INT_RAW`"]
pub type CNT_THR_EVENT_U0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U0_INT_RAW`"]
pub struct CNT_THR_EVENT_U0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U0_INT_RAW_W<'a> {
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
    #[doc = "Bit 7 - This is the interrupt raw bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7_int_raw(&self) -> CNT_THR_EVENT_U7_INT_RAW_R {
        CNT_THR_EVENT_U7_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6_int_raw(&self) -> CNT_THR_EVENT_U6_INT_RAW_R {
        CNT_THR_EVENT_U6_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt raw bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5_int_raw(&self) -> CNT_THR_EVENT_U5_INT_RAW_R {
        CNT_THR_EVENT_U5_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt raw bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4_int_raw(&self) -> CNT_THR_EVENT_U4_INT_RAW_R {
        CNT_THR_EVENT_U4_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt raw bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_raw(&self) -> CNT_THR_EVENT_U3_INT_RAW_R {
        CNT_THR_EVENT_U3_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt raw bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_raw(&self) -> CNT_THR_EVENT_U2_INT_RAW_R {
        CNT_THR_EVENT_U2_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt raw bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_raw(&self) -> CNT_THR_EVENT_U1_INT_RAW_R {
        CNT_THR_EVENT_U1_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This is the interrupt raw bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_raw(&self) -> CNT_THR_EVENT_U0_INT_RAW_R {
        CNT_THR_EVENT_U0_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - This is the interrupt raw bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7_int_raw(&mut self) -> CNT_THR_EVENT_U7_INT_RAW_W {
        CNT_THR_EVENT_U7_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - This is the interrupt raw bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6_int_raw(&mut self) -> CNT_THR_EVENT_U6_INT_RAW_W {
        CNT_THR_EVENT_U6_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - This is the interrupt raw bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5_int_raw(&mut self) -> CNT_THR_EVENT_U5_INT_RAW_W {
        CNT_THR_EVENT_U5_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - This is the interrupt raw bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4_int_raw(&mut self) -> CNT_THR_EVENT_U4_INT_RAW_W {
        CNT_THR_EVENT_U4_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - This is the interrupt raw bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_raw(&mut self) -> CNT_THR_EVENT_U3_INT_RAW_W {
        CNT_THR_EVENT_U3_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - This is the interrupt raw bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_raw(&mut self) -> CNT_THR_EVENT_U2_INT_RAW_W {
        CNT_THR_EVENT_U2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - This is the interrupt raw bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_raw(&mut self) -> CNT_THR_EVENT_U1_INT_RAW_W {
        CNT_THR_EVENT_U1_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - This is the interrupt raw bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_raw(&mut self) -> CNT_THR_EVENT_U0_INT_RAW_W {
        CNT_THR_EVENT_U0_INT_RAW_W { w: self }
    }
}