#[doc = "Reader of register PCNT_U7_CONF2_REG"]
pub type R = crate::R<u32, super::PCNT_U7_CONF2_REG>;
#[doc = "Writer for register PCNT_U7_CONF2_REG"]
pub type W = crate::W<u32, super::PCNT_U7_CONF2_REG>;
#[doc = "Register PCNT_U7_CONF2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNT_U7_CONF2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT_CNT_L_LIM_U7`"]
pub type PCNT_CNT_L_LIM_U7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCNT_CNT_L_LIM_U7`"]
pub struct PCNT_CNT_L_LIM_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_L_LIM_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_H_LIM_U7`"]
pub type PCNT_CNT_H_LIM_U7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCNT_CNT_H_LIM_U7`"]
pub struct PCNT_CNT_H_LIM_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_H_LIM_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit7."]
    #[inline(always)]
    pub fn pcnt_cnt_l_lim_u7(&self) -> PCNT_CNT_L_LIM_U7_R {
        PCNT_CNT_L_LIM_U7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit7."]
    #[inline(always)]
    pub fn pcnt_cnt_h_lim_u7(&self) -> PCNT_CNT_H_LIM_U7_R {
        PCNT_CNT_H_LIM_U7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit7."]
    #[inline(always)]
    pub fn pcnt_cnt_l_lim_u7(&mut self) -> PCNT_CNT_L_LIM_U7_W {
        PCNT_CNT_L_LIM_U7_W { w: self }
    }
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit7."]
    #[inline(always)]
    pub fn pcnt_cnt_h_lim_u7(&mut self) -> PCNT_CNT_H_LIM_U7_W {
        PCNT_CNT_H_LIM_U7_W { w: self }
    }
}
