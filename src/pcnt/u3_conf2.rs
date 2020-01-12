#[doc = "Reader of register U3_CONF2"]
pub type R = crate::R<u32, super::U3_CONF2>;
#[doc = "Writer for register U3_CONF2"]
pub type W = crate::W<u32, super::U3_CONF2>;
#[doc = "Register U3_CONF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::U3_CONF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_L_LIM_U3`"]
pub type CNT_L_LIM_U3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_L_LIM_U3`"]
pub struct CNT_L_LIM_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_L_LIM_U3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNT_H_LIM_U3`"]
pub type CNT_H_LIM_U3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_H_LIM_U3`"]
pub struct CNT_H_LIM_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_H_LIM_U3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit3."]
    #[inline(always)]
    pub fn cnt_l_lim_u3(&self) -> CNT_L_LIM_U3_R {
        CNT_L_LIM_U3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit3."]
    #[inline(always)]
    pub fn cnt_h_lim_u3(&self) -> CNT_H_LIM_U3_R {
        CNT_H_LIM_U3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit3."]
    #[inline(always)]
    pub fn cnt_l_lim_u3(&mut self) -> CNT_L_LIM_U3_W {
        CNT_L_LIM_U3_W { w: self }
    }
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit3."]
    #[inline(always)]
    pub fn cnt_h_lim_u3(&mut self) -> CNT_H_LIM_U3_W {
        CNT_H_LIM_U3_W { w: self }
    }
}
