#[doc = "Reader of register PCNT_U1_CONF1_REG"]
pub type R = crate::R<u32, super::PCNT_U1_CONF1_REG>;
#[doc = "Writer for register PCNT_U1_CONF1_REG"]
pub type W = crate::W<u32, super::PCNT_U1_CONF1_REG>;
#[doc = "Register PCNT_U1_CONF1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNT_U1_CONF1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT_CNT_THRES1_U1`"]
pub type PCNT_CNT_THRES1_U1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCNT_CNT_THRES1_U1`"]
pub struct PCNT_CNT_THRES1_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_THRES1_U1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCNT_CNT_THRES0_U1`"]
pub type PCNT_CNT_THRES0_U1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCNT_CNT_THRES0_U1`"]
pub struct PCNT_CNT_THRES0_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CNT_THRES0_U1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit1."]
    #[inline(always)]
    pub fn pcnt_cnt_thres1_u1(&self) -> PCNT_CNT_THRES1_U1_R {
        PCNT_CNT_THRES1_U1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit1."]
    #[inline(always)]
    pub fn pcnt_cnt_thres0_u1(&self) -> PCNT_CNT_THRES0_U1_R {
        PCNT_CNT_THRES0_U1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit1."]
    #[inline(always)]
    pub fn pcnt_cnt_thres1_u1(&mut self) -> PCNT_CNT_THRES1_U1_W {
        PCNT_CNT_THRES1_U1_W { w: self }
    }
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit1."]
    #[inline(always)]
    pub fn pcnt_cnt_thres0_u1(&mut self) -> PCNT_CNT_THRES0_U1_W {
        PCNT_CNT_THRES0_U1_W { w: self }
    }
}
