import { Button } from "@/components/ui/button";
import { ButtonGroup } from "@/components/ui/button-group";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle, } from "@/components/ui/card";
import { Field, FieldDescription, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input"

export function ConfigureScoring() {
    return (
        <div className="flex w-full h-full p-4">
            <Card className="flex grow h-full">
                <CardHeader>
                    <CardTitle>
                        Scoring
                    </CardTitle>

                    <CardDescription>
                        Toggle, configure, edit, and view scoring behavior
                    </CardDescription>


                    <CardAction>
                        <ButtonGroup>
                            <Button className="hover:cursor-pointer">
                                Pause
                            </Button>

                            <Button className="hover:cursor-pointer">
                                Reset
                            </Button>

                            <Button className="hover:cursor-pointer">
                                Export
                            </Button>
                        </ButtonGroup>
                    </CardAction>
                </CardHeader>

                <CardContent className="flex flex-col w-full items-center">
                    <FieldGroup className="flex flex-col gap-4">
                        <Field>
                            <FieldLabel>Target Round Time</FieldLabel>
                            <Input required type="text" inputMode="numeric" placeholder="30" />
                            <FieldDescription>Time in seconds to target for round duration. Note that this can be exceeded under heavy load, leading to longer than expected rounds.</FieldDescription>
                        </Field>

                        <Field>
                            <FieldLabel>Maximum Round Time</FieldLabel>
                            <Input required type="text" inputMode="numeric" placeholder="28" />
                            <FieldDescription>Maximum duration that a round can run for before forcefully terminating check threads. Best to have this be below Target Round Time.</FieldDescription>
                        </Field>

                        <Field>
                            <FieldLabel>Max Concurrent Checks</FieldLabel>
                            <Input required type="text" inputMode="numeric" placeholder="15" />
                            <FieldDescription>Maximum number of checks that can occur simultaneously - best to leave this default unless you know what you're doing.</FieldDescription>
                        </Field>

                        <Field>
                            <FieldLabel>Global Score Multiplier</FieldLabel>
                            <Input required type="text" inputMode="numeric" placeholder="1" />
                            <FieldDescription>Global factor in which each check is multiplied with. Increment if you like seeing big numbers!</FieldDescription>
                        </Field>

                        <Field>
                            <Button type="submit">Apply</Button>
                        </Field>
                    </FieldGroup>
                </CardContent>
            </Card>
        </div>
    )
}


