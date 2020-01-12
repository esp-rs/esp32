#[doc = "Reader of register IN1"]
pub type R = crate::R<u32, super::IN1>;
#[doc = "Writer for register IN1"]
pub type W = crate::W<u32, super::IN1>;
#[doc = "Register IN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN1_DATA`"]
pub type IN1_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN1_DATA`"]
pub struct IN1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    pub fn in1_data(&self) -> IN1_DATA_R {
        IN1_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    pub fn in1_data(&mut self) -> IN1_DATA_W {
        IN1_DATA_W { w: self }
    }
}
