/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use hotpotai::{BackgroundRemover, ColorizationFactor, HotPotAI, HotPotAIError, OutputType, PictureColorizer, PictureRestorer};

pub fn app(args: Vec<String>) {
    if args.len() == 1 {
        print!("Usage: hotpotai \"input\" [-options]\n");
        print!("           (To spark your writing)\n");
        print!("   or  hotpotai -BR \"path-to-image\" \"path-to-save\" \"image-name\"\n");
        print!("           (Remove the background from images with AI in seconds)\n");
        print!("   or  hotpotai -PC \"path-to-image\" [colorization factor] \"path-to-save\" \"image-name\"\n");
        print!("           (Turning black and white photos to color in seconds)\n");
        print!("   or  hotpotai -PR \"path-to-image\" [has scratch] \"path-to-save\" \"image-name\"\n");
        print!("           (Restore, sharpen, and repair pictures with AI)\n");
        print!("where options include:\n");
        print!("    --idea\n");
        print!("    --summary\n");
        print!("    --product-name\n");
        print!("    --product-summary\n");
        print!("    --song-lyric\n");
        print!("    --writing-idea\n");
        print!("colorization factor:\n");
        print!("    12\n");
        print!("    15\n");
        print!("    18\n");
        print!("    20\n");
        print!("    25\n");
        print!("has scratch:\n");
        print!("    true\n");
        print!("    false\n")
    } else {
        match args.get(1) {
            Some(arg_one) => if arg_one.eq("-BR") {
                match args.get(2) {
                    Some(arg_two) => match args.get(3) {
                        Some(arg_three) => match args.get(4) {
                            Some(arg_four) => {
                                let hotpot = BackgroundRemover::new(arg_two);
                                match hotpot.save_image(arg_three,arg_four) {
                                    Ok(_) => print!("Image saved successfully."),
                                    Err(err) => match err {
                                        HotPotAIError::Error(_) => print!("Error: {}", err),
                                        HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                    }
                                }
                            }
                            None => print!("Error: Image name is missing")
                        },
                        None => print!("Error: Path is missing")
                    },
                    None => print!("Error: Image file is missing")
                }
            } else if arg_one.eq("-PC") {
                match args.get(2) {
                    Some(arg_two) => match args.get(3) {
                        Some(arg_three) => match args.get(4) {
                            Some(arg_four) => match args.get(5) {
                                Some(arg_five) => if arg_three.eq("12") {
                                    let hotpot = PictureColorizer::new(arg_two,ColorizationFactor::Twelve);
                                    match hotpot.save_image(arg_four,arg_five) {
                                        Ok(_) => print!("Image saved successfully."),
                                        Err(err) => match err {
                                            HotPotAIError::Error(_) => print!("Error: {}", err),
                                            HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                        }
                                    }
                                } else if arg_three.eq("15") {
                                    let hotpot = PictureColorizer::new(arg_two,ColorizationFactor::Fifteen);
                                    match hotpot.save_image(arg_four,arg_five) {
                                        Ok(_) => print!("Image saved successfully."),
                                        Err(err) => match err {
                                            HotPotAIError::Error(_) => print!("Error: {}", err),
                                            HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                        }
                                    }
                                } else if arg_three.eq("18") {
                                    let hotpot = PictureColorizer::new(arg_two,ColorizationFactor::Eighteen);
                                    match hotpot.save_image(arg_four,arg_five) {
                                        Ok(_) => print!("Image saved successfully."),
                                        Err(err) => match err {
                                            HotPotAIError::Error(_) => print!("Error: {}", err),
                                            HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                        }
                                    }
                                } else if arg_three.eq("20") {
                                    let hotpot = PictureColorizer::new(arg_two,ColorizationFactor::Twenty);
                                    match hotpot.save_image(arg_four,arg_five) {
                                        Ok(_) => print!("Image saved successfully."),
                                        Err(err) => match err {
                                            HotPotAIError::Error(_) => print!("Error: {}", err),
                                            HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                        }
                                    }
                                } else if arg_three.eq("25") {
                                    let hotpot = PictureColorizer::new(arg_two,ColorizationFactor::TwentyFive);
                                    match hotpot.save_image(arg_four,arg_five) {
                                        Ok(_) => print!("Image saved successfully."),
                                        Err(err) => match err {
                                            HotPotAIError::Error(_) => print!("Error: {}", err),
                                            HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                        }
                                    }
                                } else {
                                    print!("Error: {} is a wrong colorization factor", arg_three)
                                },
                                None => print!("Error: Image name is missing")
                            }
                            None => print!("Error: Path is missing")
                        },
                        None => print!("Error: Coloerization is missing")
                    },
                    None => print!("Error: Image file is missing")
                }
            } else if arg_one.eq("-PR") {
                match args.get(2) {
                    Some(arg_two) => match args.get(3) {
                        Some(arg_three) => match args.get(4) {
                            Some(arg_four) => match args.get(5) {
                                Some(arg_five) => if arg_three.eq("true") {
                                    let hotpot = PictureRestorer::new(arg_two, true);
                                    match hotpot.save_image(arg_four,arg_five) {
                                        Ok(_) => print!("Image saved successfully."),
                                        Err(err) => match err {
                                            HotPotAIError::Error(_) => print!("Error: {}", err),
                                            HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                        }
                                    }
                                } else if arg_three.eq("false") {
                                    let hotpot = PictureRestorer::new(arg_two, false);
                                    match hotpot.save_image(arg_four,arg_five) {
                                        Ok(_) => print!("Image saved successfully."),
                                        Err(err) => match err {
                                            HotPotAIError::Error(_) => print!("Error: {}", err),
                                            HotPotAIError::Null(_) => print!("Error: Something went wrong")
                                        }
                                    }
                                } else {
                                    print!("Error: {} not a boolean", arg_three)
                                },
                                None => print!("Error: Image name is missing")
                            }
                            None => print!("Error: Path is missing")
                        },
                        None => print!("Error: Coloerization is missing")
                    },
                    None => print!("Error: Image file is missing")
                }
            } else {
                match args.get(2) {
                    Some(arg_two) => if arg_two.eq("--idea") {
                        let hotpot = HotPotAI::new(arg_one, OutputType::Idea);
                        match hotpot.data() {
                            Ok(data) => print!("{}",data),
                            Err(err) => match err {
                                HotPotAIError::Error(_) => print!("Error: {}", err),
                                HotPotAIError::Null(_) => print!("Error: Something went wrong")
                            }
                        }
                    } else if arg_two.eq("--summary") {
                        let hotpot = HotPotAI::new(arg_one, OutputType::Summary);
                        match hotpot.data() {
                            Ok(data) => print!("{}",data),
                            Err(err) => match err {
                                HotPotAIError::Error(_) => print!("Error: {}", err),
                                HotPotAIError::Null(_) => print!("Error: Something went wrong")
                            }
                        }
                    } else if arg_two.eq("--product-name") {
                        let hotpot = HotPotAI::new(arg_one, OutputType::ProductName);
                        match hotpot.data() {
                            Ok(data) => print!("{}",data),
                            Err(err) => match err {
                                HotPotAIError::Error(_) => print!("Error: {}", err),
                                HotPotAIError::Null(_) => print!("Error: Something went wrong")
                            }
                        }
                    } else if arg_two.eq("--product-summary") {
                        let hotpot = HotPotAI::new(arg_one, OutputType::ProductSummary);
                        match hotpot.data() {
                            Ok(data) => print!("{}",data),
                            Err(err) => match err {
                                HotPotAIError::Error(_) => print!("Error: {}", err),
                                HotPotAIError::Null(_) => print!("Error: Something went wrong")
                            }
                        }
                    } else if arg_two.eq("--song-lyric") {
                        let hotpot = HotPotAI::new(arg_one, OutputType::SongLyric);
                        match hotpot.data() {
                            Ok(data) => print!("{}",data),
                            Err(err) => match err {
                                HotPotAIError::Error(_) => print!("Error: {}", err),
                                HotPotAIError::Null(_) => print!("Error: Something went wrong")
                            }
                        }
                    } else if arg_two.eq("--writing-idea") {
                        let hotpot = HotPotAI::new(arg_one, OutputType::WritingIdea);
                        match hotpot.data() {
                            Ok(data) => print!("{}",data),
                            Err(err) => match err {
                                HotPotAIError::Error(_) => print!("Error: {}", err),
                                HotPotAIError::Null(_) => print!("Error: Something went wrong")
                            }
                        }
                    } else {
                        print!("Error: {} is a wrong option", arg_two)
                    },
                    None => print!("Error: option is missing")
                }
            },
            None => {
                print!("Usage: hotpotai \"input\" [-options]\n");
                print!("           (To spark your writing)\n");
                print!("   or  hotpotai -BR \"path-to-image\" \"path-to-save\" \"image-name\"\n");
                print!("           (Remove the background from images with AI in seconds)\n");
                print!("   or  hotpotai -PC \"path-to-image\" [colorization factor] \"path-to-save\" \"image-name\"\n");
                print!("           (Turning black and white photos to color in seconds)\n");
                print!("   or  hotpotai -PR \"path-to-image\" [has scratch] \"path-to-save\" \"image-name\"\n");
                print!("           (Restore, sharpen, and repair pictures with AI)\n");
                print!("where options include:\n");
                print!("    --idea\n");
                print!("    --summary\n");
                print!("    --product-name\n");
                print!("    --product-summary\n");
                print!("    --song-lyric\n");
                print!("    --writing-idea\n");
                print!("colorization factor:\n");
                print!("    12\n");
                print!("    15\n");
                print!("    18\n");
                print!("    20\n");
                print!("    25\n");
                print!("has scratch:\n");
                print!("    true\n");
                print!("    false\n")
            }
        }
    }
}