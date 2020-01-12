#[doc = "Reader of register CMD_INFOR1"]
pub type R = crate::R<u32, super::CMD_INFOR1>;
#[doc = "Writer for register CMD_INFOR1"]
pub type W = crate::W<u32, super::CMD_INFOR1>;
#[doc = "Register CMD_INFOR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_INFOR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_CONTENT1`"]
pub type CMD_CONTENT1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CMD_CONTENT1`"]
pub struct CMD_CONTENT1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CONTENT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmd_content1(&self) -> CMD_CONTENT1_R {
        CMD_CONTENT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmd_content1(&mut self) -> CMD_CONTENT1_W {
        CMD_CONTENT1_W { w: self }
    }
}