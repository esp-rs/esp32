#[doc = "Reader of register CK8M_TICK_CONF"]
pub type R = crate::R<u32, super::CK8M_TICK_CONF>;
#[doc = "Writer for register CK8M_TICK_CONF"]
pub type W = crate::W<u32, super::CK8M_TICK_CONF>;
#[doc = "Register CK8M_TICK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CK8M_TICK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_CK8M_TICK_NUM`"]
pub type APB_CTRL_CK8M_TICK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_CK8M_TICK_NUM`"]
pub struct APB_CTRL_CK8M_TICK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CK8M_TICK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_ctrl_ck8m_tick_num(&self) -> APB_CTRL_CK8M_TICK_NUM_R {
        APB_CTRL_CK8M_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_ctrl_ck8m_tick_num(&mut self) -> APB_CTRL_CK8M_TICK_NUM_W {
        APB_CTRL_CK8M_TICK_NUM_W { w: self }
    }
}
