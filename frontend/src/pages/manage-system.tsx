import { Button } from "@/components/ui/button";
import { ButtonGroup } from "@/components/ui/button-group";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle, } from "@/components/ui/card";
import { Field, FieldDescription, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input"
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";

export function ManageSystem() {
    return (
        <div className="flex w-full h-full p-4">
            <Card className="flex grow h-full">
                <CardHeader>
                    <CardTitle>
                        System
                    </CardTitle>

                    <CardDescription>
                        Manage, restart, and shutdown the ARSE service
                    </CardDescription>


                    <CardAction>
                        <ButtonGroup>
                            <Button className="hover:cursor-pointer">
                                Restart
                            </Button>

                            <Button className="hover:cursor-pointer">
                                Shutdown
                            </Button>
                        </ButtonGroup>
                    </CardAction>
                </CardHeader>

                <CardContent className="flex flex-col w-full items-center">
                    <FieldGroup className="flex flex-col gap-4">
                        <Field>
                            <FieldLabel>Network Interface</FieldLabel>
                            <Select required>
                                <SelectTrigger>
                                    <SelectValue placeholder="127.0.0.1" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="127.0.0.1">127.0.0.1</SelectItem>
                                        <SelectItem value="172.18.0.1">172.18.0.1</SelectItem>
                                        <SelectItem value="172.19.0.1">172.19.0.1</SelectItem>
                                        <SelectItem value="172.21.0.1">172.21.0.1</SelectItem>
                                        <SelectItem value="0.0.0.0">0.0.0.0</SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                            <FieldDescription>Choose the network interface that ARSE will bind to. Note that this can cause service interruptions, and can potentially render ARSE inaccessible.</FieldDescription>
                        </Field>


                        <Field>
                            <FieldLabel>Network Port</FieldLabel>
                            <Input required type="text" inputMode="numeric" placeholder="80" />
                            <FieldDescription>Choose the port that ARSE will listen on. Changing this can cause service interruptions.</FieldDescription>
                        </Field>

                        <Field>
                            <FieldLabel>Database Path</FieldLabel>
                            <Input required type="text" placeholder="/home/arse/arse.db" />
                            <FieldDescription>The path to the SQLite database used by ARSE. If the file pointed to by this value is non-existent, ARSE will create it and copy the current database to that location.</FieldDescription>
                        </Field>

                        <Field>
                            <Button type="submit">Apply</Button>
                        </Field>
                    </FieldGroup>
                </CardContent>
            </Card>
        </div >
    )
}
