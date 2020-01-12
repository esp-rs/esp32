#[doc = "Reader of register COMD3"]
pub type R = crate::R<u32, super::COMD3>;
#[doc = "Writer for register COMD3"]
pub type W = crate::W<u32, super::COMD3>;
#[doc = "Register COMD3 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMAND3_DONE`"]
pub type COMMAND3_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMMAND3_DONE`"]
pub struct COMMAND3_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND3_DONE_W<'a> {
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
#[doc = "Reader of field `COMMAND3`"]
pub type COMMAND3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMAND3`"]
pub struct COMMAND3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - When command3 is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn command3_done(&self) -> COMMAND3_DONE_R {
        COMMAND3_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13 - This is the content of command3. It consists of three part. op_code is the command 0: RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn command3(&self) -> COMMAND3_R {
        COMMAND3_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - When command3 is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn command3_done(&mut self) -> COMMAND3_DONE_W {
        COMMAND3_DONE_W { w: self }
    }
    #[doc = "Bits 0:13 - This is the content of command3. It consists of three part. op_code is the command 0: RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn command3(&mut self) -> COMMAND3_W {
        COMMAND3_W { w: self }
    }
}
