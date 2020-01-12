#[doc = "Reader of register COMD4"]
pub type R = crate::R<u32, super::COMD4>;
#[doc = "Writer for register COMD4"]
pub type W = crate::W<u32, super::COMD4>;
#[doc = "Register COMD4 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMD4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND4_DONE`"]
pub type COMMAND4_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMMAND4_DONE`"]
pub struct COMMAND4_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND4_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `COMMAND4`"]
pub type COMMAND4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND4`"]
pub struct COMMAND4_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - When command4 is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13 - This is the content of command4. It consists of three part. op_code is the command 0: RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - When command4 is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn command4_done(&mut self) -> COMMAND4_DONE_W {
        COMMAND4_DONE_W { w: self }
    }
    #[doc = "Bits 0:13 - This is the content of command4. It consists of three part. op_code is the command 0: RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn command4(&mut self) -> COMMAND4_W {
        COMMAND4_W { w: self }
    }
}
