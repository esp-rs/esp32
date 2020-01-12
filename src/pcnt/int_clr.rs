#[doc = "Reader of register INT_CLR"]
pub type R = crate::R<u32, super::INT_CLR>;
#[doc = "Writer for register INT_CLR"]
pub type W = crate::W<u32, super::INT_CLR>;
#[doc = "Register INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_THR_EVENT_U7_INT_CLR`"]
pub type CNT_THR_EVENT_U7_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U7_INT_CLR`"]
pub struct CNT_THR_EVENT_U7_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U7_INT_CLR_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U6_INT_CLR`"]
pub type CNT_THR_EVENT_U6_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U6_INT_CLR`"]
pub struct CNT_THR_EVENT_U6_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U6_INT_CLR_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U5_INT_CLR`"]
pub type CNT_THR_EVENT_U5_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U5_INT_CLR`"]
pub struct CNT_THR_EVENT_U5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U5_INT_CLR_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U4_INT_CLR`"]
pub type CNT_THR_EVENT_U4_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U4_INT_CLR`"]
pub struct CNT_THR_EVENT_U4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U4_INT_CLR_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U3_INT_CLR`"]
pub type CNT_THR_EVENT_U3_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U3_INT_CLR`"]
pub struct CNT_THR_EVENT_U3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U3_INT_CLR_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U2_INT_CLR`"]
pub type CNT_THR_EVENT_U2_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U2_INT_CLR`"]
pub struct CNT_THR_EVENT_U2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U2_INT_CLR_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U1_INT_CLR`"]
pub type CNT_THR_EVENT_U1_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U1_INT_CLR`"]
pub struct CNT_THR_EVENT_U1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U1_INT_CLR_W<'a> {
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
#[doc = "Reader of field `CNT_THR_EVENT_U0_INT_CLR`"]
pub type CNT_THR_EVENT_U0_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_THR_EVENT_U0_INT_CLR`"]
pub struct CNT_THR_EVENT_U0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THR_EVENT_U0_INT_CLR_W<'a> {
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
    #[doc = "Bit 7 - Set this bit to clear channel7 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u7_int_clr(&self) -> CNT_THR_EVENT_U7_INT_CLR_R {
        CNT_THR_EVENT_U7_INT_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to clear channel6 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u6_int_clr(&self) -> CNT_THR_EVENT_U6_INT_CLR_R {
        CNT_THR_EVENT_U6_INT_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to clear channel5 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u5_int_clr(&self) -> CNT_THR_EVENT_U5_INT_CLR_R {
        CNT_THR_EVENT_U5_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to clear channel4 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u4_int_clr(&self) -> CNT_THR_EVENT_U4_INT_CLR_R {
        CNT_THR_EVENT_U4_INT_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to clear channel3 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_clr(&self) -> CNT_THR_EVENT_U3_INT_CLR_R {
        CNT_THR_EVENT_U3_INT_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear channel2 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_clr(&self) -> CNT_THR_EVENT_U2_INT_CLR_R {
        CNT_THR_EVENT_U2_INT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to clear channel1 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_clr(&self) -> CNT_THR_EVENT_U1_INT_CLR_R {
        CNT_THR_EVENT_U1_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to clear channel0 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_clr(&self) -> CNT_THR_EVENT_U0_INT_CLR_R {
        CNT_THR_EVENT_U0_INT_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Set this bit to clear channel7 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u7_int_clr(&mut self) -> CNT_THR_EVENT_U7_INT_CLR_W {
        CNT_THR_EVENT_U7_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear channel6 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u6_int_clr(&mut self) -> CNT_THR_EVENT_U6_INT_CLR_W {
        CNT_THR_EVENT_U6_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to clear channel5 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u5_int_clr(&mut self) -> CNT_THR_EVENT_U5_INT_CLR_W {
        CNT_THR_EVENT_U5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear channel4 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u4_int_clr(&mut self) -> CNT_THR_EVENT_U4_INT_CLR_W {
        CNT_THR_EVENT_U4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to clear channel3 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_clr(&mut self) -> CNT_THR_EVENT_U3_INT_CLR_W {
        CNT_THR_EVENT_U3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear channel2 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_clr(&mut self) -> CNT_THR_EVENT_U2_INT_CLR_W {
        CNT_THR_EVENT_U2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to clear channel1 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_clr(&mut self) -> CNT_THR_EVENT_U1_INT_CLR_W {
        CNT_THR_EVENT_U1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to clear channel0 event interrupt."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_clr(&mut self) -> CNT_THR_EVENT_U0_INT_CLR_W {
        CNT_THR_EVENT_U0_INT_CLR_W { w: self }
    }
}
