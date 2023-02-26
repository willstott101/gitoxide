use std::io;

use crate::client::{ExtendedBufRead, MessageKind, WriteMode};

/// A [`Write`][io::Write] implementation optimized for writing packet lines.
/// A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for
/// obtaining the response.
pub struct RequestWriter<'a> {
    on_into_read: MessageKind,
    writer: gix_packetline::Writer<Box<dyn io::Write + 'a>>,
    reader: Box<dyn ExtendedBufRead + Unpin + 'a>,
}

impl<'a> io::Write for RequestWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// methods with bonds to IO
impl<'a> RequestWriter<'a> {
    /// Create a new instance from a `writer` (commonly a socket), a `reader` into which to transform once the
    /// writes are finished, along with configuration for the `write_mode` and information about which message to write
    /// when this instance is converted into a `reader` to read the request's response.
    pub fn new_from_bufread<W: io::Write + 'a>(
        writer: W,
        reader: Box<dyn ExtendedBufRead + Unpin + 'a>,
        write_mode: WriteMode,
        on_into_read: MessageKind,
    ) -> Self {
        let mut writer = gix_packetline::Writer::new(Box::new(writer) as Box<dyn io::Write>);
        match write_mode {
            WriteMode::Binary => writer.enable_binary_mode(),
            WriteMode::OneLfTerminatedLinePerWriteCall => writer.enable_text_mode(),
        }
        RequestWriter {
            on_into_read,
            writer,
            reader,
        }
    }

    /// Write the given message as packet line.
    pub fn write_message(&mut self, message: MessageKind) -> io::Result<()> {
        match message {
            MessageKind::Flush => gix_packetline::PacketLineRef::Flush.write_to(self.writer.inner_mut()),
            MessageKind::Delimiter => gix_packetline::PacketLineRef::Delimiter.write_to(self.writer.inner_mut()),
            MessageKind::ResponseEnd => gix_packetline::PacketLineRef::ResponseEnd.write_to(self.writer.inner_mut()),
            MessageKind::Text(t) => gix_packetline::TextRef::from(t).write_to(self.writer.inner_mut()),
        }
        .map(|_| ())
    }

    /// Discard the ability to write and turn this instance into the reader for obtaining the other side's response.
    ///
    /// Doing so will also write the message type this instance was initialized with.
    pub fn into_read(mut self) -> std::io::Result<Box<dyn ExtendedBufRead + Unpin + 'a>> {
        self.write_message(self.on_into_read)?;
        Ok(self.reader)
    }

    /// Dissolve this instance into its write and read handles without any message-writing side-effect as in [RequestWriter::into_read()].
    ///
    /// Furthermore, the writer will not encode everything it writes as packetlines, but write everything verbatim into the
    /// underlying channel.
    ///
    /// # Note
    ///
    /// It's of utmost importance to drop the request writer before reading the response as these might be inter-dependent, depending on
    /// the underlying transport mechanism. Failure to do so may result in a deadlock depending on how the write and read mechanism
    /// is implemented.
    pub fn into_parts(self) -> (Box<dyn io::Write + 'a>, Box<dyn ExtendedBufRead + Unpin + 'a>) {
        (self.writer.into_inner(), self.reader)
    }
}
