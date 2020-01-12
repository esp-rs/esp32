#[doc = "Reader of register U0_CONF1"]
pub type R = crate::R<u32, super::U0_CONF1>;
#[doc = "Writer for register U0_CONF1"]
pub type W = crate::W<u32, super::U0_CONF1>;
#[doc = "Register U0_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::U0_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_THRES1_U0`"]
pub type CNT_THRES1_U0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_THRES1_U0`"]
pub struct CNT_THRES1_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THRES1_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNT_THRES0_U0`"]
pub type CNT_THRES0_U0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_THRES0_U0`"]
pub struct CNT_THRES0_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THRES0_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres1_u0(&self) -> CNT_THRES1_U0_R {
        CNT_THRES1_U0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres0_u0(&self) -> CNT_THRES0_U0_R {
        CNT_THRES0_U0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres1_u0(&mut self) -> CNT_THRES1_U0_W {
        CNT_THRES1_U0_W { w: self }
    }
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres0_u0(&mut self) -> CNT_THRES0_U0_W {
        CNT_THRES0_U0_W { w: self }
    }
}
