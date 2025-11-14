ILDA Image Data Transfer Format
Specification
ILDA Technical Committee
mail@laserist.org
November 16, 2014
Revision 011
ILDA Image Data Transfer Format Specification
Contents
1. ILDA Image Data Transfer Format 4
1.1. Scope of the ILDA File Format . . . . . . . . . . . . . . . . . . . . . . . 4
2. Introduction 5
2.1. Nomenclature and Structure . . . . . . . . . . . . . . . . . . . . . . . . 5
2.2. Binary vs. ASCII . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5
2.3. Byte Order . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5
2.4. Point Data in ILDA Files . . . . . . . . . . . . . . . . . . . . . . . . . . . 5
2.5. Colors in ILDA Files . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 5
3. File Structure 6
3.1. Layout . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 6
3.2. Format Codes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 6
3.3. Application Requirements . . . . . . . . . . . . . . . . . . . . . . . . . . 7
3.4. Color Palette Handling . . . . . . . . . . . . . . . . . . . . . . . . . . . 7
4. Header Section 8
4.1. Structure . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 8
4.2. Field Description . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 8
4.2.1. "ILDA" . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 8
4.2.2. Reserved . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 8
4.2.3. Format Code . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 9
4.2.4. Frame or Color Palette Name . . . . . . . . . . . . . . . . . . . . 9
4.2.5. Company Name . . . . . . . . . . . . . . . . . . . . . . . . . . . 9
4.2.6. Number of Records . . . . . . . . . . . . . . . . . . . . . . . . . 9
4.2.7. Frame or Color Palette Number . . . . . . . . . . . . . . . . . . . 9
4.2.8. Total Frames in Sequence or 0 . . . . . . . . . . . . . . . . . . . 9
4.2.9. Projector Number . . . . . . . . . . . . . . . . . . . . . . . . . . 10
5. Data Records 10
5.1. Data Record Structures . . . . . . . . . . . . . . . . . . . . . . . . . . . 10
5.1.1. Format 0 – 3D Coordinates with Indexed Color . . . . . . . . . . 10
5.1.2. Format 1 – 2D Coordinates with Indexed Color . . . . . . . . . . 10
5.1.3. Format 2 – Color Palette . . . . . . . . . . . . . . . . . . . . . . 11
5.1.4. Format 4 – 3D Coordinates with True Color . . . . . . . . . . . . 11
5.1.5. Format 5 – 2D Coordinates with True Color . . . . . . . . . . . . 12
5.2. Data Field Description . . . . . . . . . . . . . . . . . . . . . . . . . . . . 12
5.2.1. X Coordinate . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 12
5.2.2. Y Coordinate . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
5.2.3. Z Coordinate . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
Page 2 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
5.2.4. Status Code . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
5.2.5. Color Index . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
5.2.6. Blue Color Component . . . . . . . . . . . . . . . . . . . . . . . 14
5.2.7. Green Color Component . . . . . . . . . . . . . . . . . . . . . . 14
5.2.8. Red Color Component . . . . . . . . . . . . . . . . . . . . . . . 14
6. Revision History 15
7. Contributors 15
8. Copyright 16
Appendix 17
A. Suggested Default Color Palette 17
Revision 011 
c 2014 International Laser Display Association Page 3
ILDA Image Data Transfer Format Specification
1. ILDA Image Data Transfer Format
This technical standard describes the official International Laser Display Association
Data Transfer Format for exchanging laser show frames between systems. It has been
developed by ILDA’s Technical Committee.
The official name of this standard is “ILDA Image Data Transfer Format”.
You can obtain frames from any program that correctly writes ILDA-format files, and
transparently load them directly into any system that can load ILDA-format files. Similarly, you can save frames in ILDA format, to sell or trade with users of other systems
that read ILDA format.
1.1. Scope of the ILDA File Format
The ILDA format is intended for frame exchange purposes only. A laser system is free
to read and write its own proprietary format that best meets its features and requirements.
It is not optimized for space or speed, and it is not currently concerned with display
issues such as point output rate. Also, the format does not include show information
such as timing of frames. Generally, the highest function the ILDA format can provide is
a sequence of frames which play back to form an animation.
Page 4 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
2. Introduction
2.1. Nomenclature and Structure
Throughout this document, the word “SHALL” is used in capitals to stress required conformance with the ILDA Format. The word “SHOULD” in capitals indicates suggested
conformance.
2.2. Binary vs. ASCII
The terms “binary 0” or “binary 1” refer to bit codes 0000 0000 and 0000 0001. They are
used to avoid confusion with the ASCII characters “0” or “1”.
2.3. Byte Order
For values which span more than a single byte, the multiple byte ordering followed
SHALL be that of the big endian standard. The most significant byte will occur first,
the least significant byte last.
2.4. Point Data in ILDA Files
The ILDA format is intended for “point-oriented” frames only rather than “vector-oriented”
frames. This means the data in an ILDA file is interpreted as data samples which are
directly sent to the galvanometer scanners used in laser projectors. The data is NOT
raw vector information which needs further processing.
2.5. Colors in ILDA Files
Assume that the RGB color values specified in this standard are linear and are color
balanced. For linearity, this is visual linearity: a color value of 127 (50 %) appears half
as bright to the eye as a setting of 255 (100 %).
Revision 011 
c 2014 International Laser Display Association Page 5
ILDA Image Data Transfer Format Specification
3. File Structure
3.1. Layout
An ILDA file consists of sections which either contain a frame or a color palette. Each
section consists of a fixed length header followed by a variable number of data records,
which are either frame points or color palette colors.
Header



Header
Data Record #0
followed by up to 65534
Data Records
❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤❤
❤❤❤
❤❤❤❤
❤❤❤
❤❤❤❤
❤❤❤
❤❤❤❤
❤❤❤
❤❤❤❤
❤❤
Section
(frame or
color palette)






Data
.
.
.
Frame Header with Number of Records = 0
Last
Section






Header
Figure 1: ILDA File Structure
The “Number of Records” field in the header defines how many data records will follow
the header.
The end of the file is marked by a header with frame format code (Format 0, 1, 4 or 5)
and zero number of records.
3.2. Format Codes
The type and data format of the section is defined by the format code. There are five
different formats currently defined:
Page 6 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
• Format 0 – 3D Coordinates with Indexed Color
• Format 1 – 2D Coordinates with Indexed Color
• Format 2 – Color Palette for Indexed Color Frames
• Format 4 – 3D Coordinates with True Color
• Format 5 – 2D Coordinates with True Color
Format 3 was proposed within the ILDA Technical Committee but was never approved.
Therefore, format 3 is omitted in this ILDA standard.
Formats 0, 1, 4 and 5 define point data. Each point includes X and Y coordinates, and
color information. The 3D formats 0 and 4 also include Z (depth) information.
The indexed color formats 0 and 1 use a data format where each point has a Color Index
between 0 and 255 used as an index into a color palette. Format 2 specifies the color
palette for use with indexed color frames. The true color formats 4 and 5 use a red,
green and blue color component of 8 bits for each point. ILDA files may contain a mix of
frames with several different format codes.
3.3. Application Requirements
An application which reads ILDA format files SHALL be able to read all five current
formats (0, 1, 2, 4 and 5).
Newly created applications SHOULD primarily use the true color frame formats.
For compatibility with older versions of the ILDA file format applications SHOULD be able
to write one of the indexed color frame formats, optionally including the color palette.
3.4. Color Palette Handling
For each projector there is a color palette which is used for indexed color frames.
The color palette used for a projector can be set using a format 2 color palette section.
The color palette will then be used for all following frames for that projector. If another
format 2 section is encountered for that projector, it will replace the projector’s current
color palette.
Often ILDA files contain indexed color frames without a format 2 color table preceding
them. For this case the color palette has to be initialized to a user-defined color palette.
One possible palette is given in Appendix A of this standard.
Revision 011 
c 2014 International Laser Display Association Page 7
ILDA Image Data Transfer Format Specification
4. Header Section
4.1. Structure
The header has a fixed size of 32 bytes and the following structure:
8151631 7 0
"ILDA"
(1 – 4)
Reserved
(5 – 7)
Format Code
(8)
Frame or Color Palette Name
(9-16)
Company Name
(17 – 24)
Number of Records
(25 – 26)
Frame or Color Palette Number
(27 – 28)
Total Frames or 0
(29 – 30)
Projector Number
(31)
Reserved
(32)
Figure 2: Header Structure
Byte numbers in parenthesis.
4.2. Field Description
4.2.1. "ILDA"
Bytes 1 – 4. The ASCII letters ILDA, identifying an ILDA format header.
4.2.2. Reserved
Bytes 5 – 7 and 32. Reserved for future use. When writing a file, this SHALL be set to
0. When reading a file, do not test the value of these bytes.
Page 8 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
4.2.3. Format Code
Byte 8. One of the format codes defined in the Format Codes section.
4.2.4. Frame or Color Palette Name
Bytes 9 – 16. Eight ASCII characters with the name of this frame or color palette. If a
binary zero is encountered, than any characters following the zero SHALL be ignored.
4.2.5. Company Name
Bytes 17 – 24. Eight ASCII characters with the name of the company who created the
frame. If a binary zero is encountered, than any characters following the zero SHALL be
ignored.
4.2.6. Number of Records
Bytes 25 – 26. Total number of data records (points or colors) that will follow this header
expressed as an unsigned integer (0 – 65535).
If the number of records is 0, then this is to be taken as the end of file header and no
more data will follow this header.
For color palettes, the number of records SHALL be between 2 and 256.
4.2.7. Frame or Color Palette Number
Bytes 27 – 28. If the frame is part of a group such as an animation sequence, this
represents the frame number. Counting begins with frame 0. Range is 0 – 65534.
4.2.8. Total Frames in Sequence or 0
Bytes 29 – 30. Total frames in this group or sequence. Range is 1 – 65535. For color
palettes this SHALL be 0.
Revision 011 
c 2014 International Laser Display Association Page 9
ILDA Image Data Transfer Format Specification
4.2.9. Projector Number
Byte 31. The projector number that this frame is to be displayed on. Range is 0 – 255.
For single projector files this SHOULD be set 0.
5. Data Records
5.1. Data Record Structures
5.1.1. Format 0 – 3D Coordinates with Indexed Color
Format 0 records have a size of 8 bytes and the following structure:
815 7 0
X Coordinate (1 – 2)
Y Coordinate (3 – 4)
Z Coordinate (5 – 6)
Status Code (7) Color Index (8)
Figure 3: Structure of Point Format 0 -
3D Coordinates with “Indexed Color”
Byte numbers in parenthesis.
5.1.2. Format 1 – 2D Coordinates with Indexed Color
Format 1 records have a size of 6 bytes and the following structure:
Page 10 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
815 7 0
X Coordinate (1 – 2)
Y Coordinate (3 – 4)
Status Code (5) Color Index (6)
Figure 4: Structure of Point Format 1 -
2D Coordinates with “Indexed Color”
Byte numbers in parenthesis.
5.1.3. Format 2 – Color Palette
Format 2 records have a size of 3 bytes and the following structure:
7 0
Red (1)
Green (2)
Blue (3)
Figure 5: Structure of Format 2 -
Color Palette
Byte numbers in parenthesis.
5.1.4. Format 4 – 3D Coordinates with True Color
Format 4 records have a size of 10 bytes and the following structure:
Revision 011 
c 2014 International Laser Display Association Page 11
ILDA Image Data Transfer Format Specification
815 7 0
X Coordinate (1 – 2)
Y Coordinate (3 – 4)
Z Coordinate (5 – 6)
Status Code (7) Blue (8)
Green (9) Red (10)
Figure 6: Structure of Point Format 4 -
3D Coordinates with True Color
Byte numbers in parenthesis.
5.1.5. Format 5 – 2D Coordinates with True Color
Format 5 records have a size of 8 bytes and the following structure:
815 7 0
X Coordinate (1-2)
Y Coordinate (3-4)
Status Code (5) Blue (6)
Green (7) Red (8)
Figure 7: Structure of Point Format 5 -
2D Coordinates with True Color
Byte numbers in parenthesis.
5.2. Data Field Description
5.2.1. X Coordinate
A 16-bit binary twos complement (signed) integer.
Extreme left is -32768; extreme right is +32767. (All directions referenced to front projection.)
Page 12 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
5.2.2. Y Coordinate
A 16-bit binary twos complement (signed) integer.
Extreme bottom is -32768; extreme top is +32767.
5.2.3. Z Coordinate
A 16-bit binary twos complement (signed) integer.
Extreme rear (away from viewer; behind screen) is -32768; extreme front (towards
viewer; in front of screen) is +32767.
5.2.4. Status Code
7 6 5 0
Last Point
(Bit 7)
Blanking
(Bit 6) 0
Figure 8: Status Code Format
Bit 7 (MSB) – Last Point Bit: This bit SHALL be set to 0 for all points except the last
point of the image.
Bit 6 – Blanking Bit: If this is a 1, then the laser is off (blank). If this is a 0, then the
laser is on (draw). Note that all systems SHALL write this bit, even if a particular system
uses the color index for blanking/color information.
When reading files, the blanking bit takes precedence over the color from the color
palette or the points RGB values. If the blanking bit is set, all RGB values SHOULD be
treated as zero.
Bits 0 – 5: SHALL be set to 0. Do not test the value of these bits.
5.2.5. Color Index
Indicates the point’s color number. This value is used as an index into the color palette.
Revision 011 
c 2014 International Laser Display Association Page 13
ILDA Image Data Transfer Format Specification
5.2.6. Blue Color Component
This value is the point’s blue color component. A value of 0 indicates “zero brightness”
and a value of 255 indicates “maximum brightness”.
5.2.7. Green Color Component
This value is the point’s green color component. A value of 0 indicates “zero brightness”
and a value of 255 indicates “maximum brightness”.
5.2.8. Red Color Component
This value is the point’s red color component. A value of 0 indicates “zero brightness”
and a value of 255 indicates “maximum brightness”.
Page 14 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
6. Revision History
Not all versions are listed here.
• Revision 004, June 1992 – Added Format 2 with color header table data.
• Revision 005.1, July 2006 – Corrected coordinate ranges and made a minor correction in one place.
• Revision 008, March 2007 – Added Formats 4 and 5. Limited distribution as a
draft, within the ILDA Technical Committee
• Revision 009, October 2008 – No change to Formats. Extensive changes and
additions to explanatory text.
• Revision 010A, April 2013 – Major update of layout and structure of this document.
• Revision 010B, September 2013 – Minor corrections.
• Revision 010C, June 2014 – Minor corrections.
• Revision 010D, October 2014 – No changes to Formats. Renamed "Scanner Number" to "Projector Number", various minor corrections.
• Revision 010E, October 2014 – No changes to Formats. Various minor corrections, added bytes numbers to tables.
• Revision 011, November 2014 – Release Version
7. Contributors
Many individuals have contributed to ILDA standards development and this document.
These include the following contributors:
Robin Adams, RayComposer
William R. Benner, Jr., Pangolin Laser Systems, Inc.
Daniel Cohn, Technological Artisans
Steve Heminover, Aura Technologies, Inc.
Peter Jakubek, LaserAnimation Sollinger GmbH
Patrick Murphy, Pangolin Laser Systems, Inc.
Frank Plughoff, Full Spectrum Lasers
Kelly Plughoff, Full Spectrum Lasers
Matt Polak, Raven Systems Design, Inc.
Michael Sollinger, LaserAnimation Sollinger GmbH
Revision 011 
c 2014 International Laser Display Association Page 15
ILDA Image Data Transfer Format Specification
8. Copyright
This document 
c 1992-2014 International Laser Display Association. All rights reserved.
For reproduction permission contact ILDA’s Executive Director (mail@laserist.org).
Page 16 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
Appendix
A. Suggested Default Color Palette
The color palette described here was originally developed by LFI and Aura Technologies.
It contains 64 colors of the full saturated hues and white.
This color palette is used by most ILDA files that do not contain a color palette, including
the ILDA test pattern.
Table 1: Suggested Default Color Palette
Color Number Red Green Blue Color name
0 255 0 0 Red
1 255 16 0
2 255 32 0
3 255 48 0
4 255 64 0
5 255 80 0
6 255 96 0
7 255 112 0
8 255 128 0
9 255 144 0
10 255 160 0
11 255 176 0
12 255 192 0
13 255 208 0
14 255 224 0
15 255 240 0
16 255 255 0 Yellow
17 224 255 0
Revision 011 
c 2014 International Laser Display Association Page 17
ILDA Image Data Transfer Format Specification
18 192 255 0
19 160 255 0
20 128 255 0
21 96 255 0
22 64 255 0
23 32 255 0
24 0 255 0 Green
25 0 255 36
26 0 255 73
27 0 255 109
28 0 255 146
29 0 255 182
30 0 255 219
31 0 255 255 Cyan
32 0 227 255
33 0 198 255
34 0 170 255
35 0 142 255
36 0 113 255
37 0 85 255
38 0 56 255
39 0 28 255
40 0 0 255 Blue
41 32 0 255
42 64 0 255
43 96 0 255
44 128 0 255
45 160 0 255
Page 18 
c 2014 International Laser Display Association Revision 011
ILDA Image Data Transfer Format Specification
46 192 0 255
47 224 0 255
48 255 0 255 Magenta
49 255 32 255
50 255 64 255
51 255 96 255
52 255 128 255
53 255 160 255
54 255 192 255
55 255 224 255
56 255 255 255 White
57 255 224 224
58 255 192 192
59 255 160 160
60 255 128 128
61 255 96 96
62 255 64 64
63 255 32 32
Revision 011 
c 2014 International Laser Display Association Page 19