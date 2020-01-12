#[doc = "Reader of register AT_CMD_POSTCNT"]
pub type R = crate::R<u32, super::AT_CMD_POSTCNT>;
#[doc = "Writer for register AT_CMD_POSTCNT"]
pub type W = crate::W<u32, super::AT_CMD_POSTCNT>;
#[doc = "Register AT_CMD_POSTCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::AT_CMD_POSTCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POST_IDLE_NUM`"]
pub type POST_IDLE_NUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `POST_IDLE_NUM`"]
pub struct POST_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the duration time between the last at_cmd and the next data. when the duration is less than this register value it will not take the previous data as at_cmd char."]
    #[inline(always)]
    pub fn post_idle_num(&self) -> POST_IDLE_NUM_R {
        POST_IDLE_NUM_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the duration time between the last at_cmd and the next data. when the duration is less than this register value it will not take the previous data as at_cmd char."]
    #[inline(always)]
    pub fn post_idle_num(&mut self) -> POST_IDLE_NUM_W {
        POST_IDLE_NUM_W { w: self }
    }
}
